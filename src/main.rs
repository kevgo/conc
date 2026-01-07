mod arguments;
mod errors;
mod print;
mod subshell;

use errors::UserError;
use std::env;
use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;

fn main() -> ExitCode {
    let commands = arguments::parse_commands(env::args().skip(1));
    if commands.is_empty() {
        print::error(&UserError::NoCommandsProvided);
        return ExitCode::FAILURE;
    }
    let (send, receive) = mpsc::channel();

    // execute all commands concurrently
    for command in commands {
        let send_clone = send.clone();
        thread::spawn(move || {
            let _ = send_clone.send(subshell::execute_command(command.clone()));
        });
    }

    // drop the original sender so the receiver knows when all threads are done
    drop(send);

    // print results as they arrive and collect exit codes
    let mut exit_code = 0;
    for call_result in receive {
        match call_result {
            Ok(call_result) => {
                print::result(&call_result);
                exit_code = exit_code.max(call_result.exit_code());
            }
            Err(err) => {
                print::error(&err);
                exit_code = exit_code.max(1);
            }
        }
    }
    ExitCode::from(exit_code.min(255) as u8)
}
