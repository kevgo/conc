use crate::commands::Show;
use crate::subshell::{Call, CallResult};
use colored::Colorize;
use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;

pub fn run(calls: Vec<Call>, show: Show, error_on_output: bool) -> ExitCode {
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
    let mut has_output = false;
    for call_result in receive {
        match call_result {
            Ok(call_result) => {
                // Check if this command produced any output
                if !call_result.output.stdout.is_empty() || !call_result.output.stderr.is_empty() {
                    has_output = true;
                }
                print_result(&call_result, show);
                exit_code = exit_code.max(call_result.exit_code());
            }
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                exit_code = exit_code.max(1);
            }
        }
    }
    // If error_on_output is set and any command produced output, exit with code 1
    if error_on_output && has_output {
        exit_code = exit_code.max(1);
    }
    ExitCode::from(exit_code)
}

/// prints the result of a single command execution to stdout and stderr
fn print_result(call_result: &CallResult, show: Show) {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // print command name
    if call_result.output.status.success() {
        let mut command = call_result.call.to_string();
        if show.display_success() {
            command = command.bold().to_string();
        }
        let _ = writeln!(stdout, "{command}");
    } else {
        let command = call_result.call.to_string().bold().red();
        let _ = writeln!(stdout, "{command}");
    }

    if call_result.output.status.success() && !show.display_success() {
        return;
    }

    write_output(&mut stdout, &call_result.output.stdout);
    write_output(&mut stderr, &call_result.output.stderr);
}

fn write_output(writer: &mut dyn Write, output: &[u8]) {
    if !output.is_empty() {
        let _ = writer.write_all(output);
        if !output.ends_with(b"\n") {
            let _ = writer.write_all(b"\n");
        }
    }
}
