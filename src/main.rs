mod arguments;
mod config;
mod errors;
mod print;
mod subshell;

use errors::UserError;
use std::env;
use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;

fn main() -> ExitCode {
    match inner() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            print::error(&err);
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<ExitCode, UserError> {
    let (config, calls) = arguments::parse(env::args().skip(1))?;
    let (send, receive) = mpsc::channel();

    // execute all commands concurrently and let them signal via the channel when they are done
    for call in calls {
        let send_clone = send.clone();
        thread::spawn(move || {
            let _ = send_clone.send(call.run());
        });
    }

    // drop the original sender so the receiver knows when all senders are closed
    drop(send);

    // print results as they arrive and collect exit codes
    let mut exit_code = 0;
    for call_result in receive {
        match call_result {
            Ok(call_result) => {
                print::result(&call_result, &config.show);
                exit_code = exit_code.max(call_result.exit_code());
            }
            Err(err) => {
                print::error(&err);
                exit_code = exit_code.max(1);
            }
        }
    }
    Ok(ExitCode::from(exit_code.min(255) as u8))
}
