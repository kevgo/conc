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
        let txc = tx.clone();
        thread::spawn(move || {
            let call_result = subshell::execute_command(command.clone());
            let _ = txc.send(call_result);
        });
    }

    // drop the original sender so the receiver knows when all threads are done
    drop(tx);

    // print results as they arrive and collect exit codes
    let mut exit_code = 0;
    for call_result in rx {
        match call_result {
            Ok(call_result) => {
                cli::print::result(&call_result);
                if exit_code == 0 {
                    exit_code = call_result.exit_code();
                }
            }
            Err(err) => {
                cli::print::user_error(&err);
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
