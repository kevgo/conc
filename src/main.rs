mod cli;
mod errors;
mod subshell;

use std::env;
use std::sync::mpsc;
use std::thread;

fn main() {
    let commands = cli::arguments::parse_commands(env::args().into_iter());
    if commands.is_empty() {
        eprintln!("No commands provided");
        std::process::exit(1);
    }
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::with_capacity(commands.len());
    for command in commands {
        handles.push(thread::spawn(move || {
            let tx = tx.clone();
            let output = subshell::execute_command(command);
            let _ = tx.send(output);
        }));
    }

    // Drop the original sender so the receiver knows when all threads are done
    drop(tx);

    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join();
    }

    for output in rx {
        cli::print::print_success(output);
    }

    // Collect all exit codes and exit with the first non-zero code found
    if let Some(error_code) = rx.iter().find(|&code| code != 0) {
        std::process::exit(error_code);
    }
}
