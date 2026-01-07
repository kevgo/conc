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
    if config.version {
        print::version();
        return Ok(ExitCode::SUCCESS);
    }
    if config.help {
        print::help();
        return Ok(ExitCode::SUCCESS);
    }

    // here we run the given commands
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
    #[allow(clippy::cast_possible_truncation)] // we reduce the value to 255 before casting
    #[allow(clippy::cast_sign_loss)] // we get the absolute value before casting
    Ok(ExitCode::from(exit_code.min(255) as u8))
}
