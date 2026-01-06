mod cli;
mod errors;
mod subshell;

use std::env;
use std::sync::mpsc;
use std::thread;

fn main() {
    let commands = cli::arguments::parse_commands(env::args().skip(1));
    if commands.is_empty() {
        eprintln!("No commands provided");
        std::process::exit(1);
    }
    let (tx, rx) = mpsc::channel();

    for command in commands {
        let tx = tx.clone();
        thread::spawn(move || {
            let output = subshell::execute_command(command.clone());
            let _ = tx.send((command, output));
        });
    }

    // Drop the original sender so the receiver knows when all threads are done
    drop(tx);

    // Print results as they arrive and collect exit codes
    let mut exit_code = 0;
    for (command, result) in rx {
        match result {
            Ok(output) => {
                cli::print::output(&command, &output);
                if !output.status.success() {
                    if let Some(code) = output.status.code() {
                        if exit_code == 0 {
                            exit_code = code;
                        }
                    }
                }
            }
            Err(err) => {
                cli::print::error(&command, &err);
                if exit_code == 0 {
                    exit_code = 1;
                }
            }
        }
    }
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
