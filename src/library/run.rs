use super::CallResult;
use super::Show;
use crate::library::subshell;
use colored::Colorize;
use std::io::{self, Write};
use std::process::Command;
use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;

/// all information Conc needs to execute a command
#[derive(Debug)]
pub struct Executable {
    /// how the command will be displayed
    pub name: String,

    /// the command to execute
    pub command: Command,
}

/// named arguments for the `run` function
#[derive(Debug)]
pub struct RunArgs {
    /// the commands to execute concurrently
    pub executables: Vec<Executable>,

    /// whether to error if any command produces output
    pub error_on_output: bool,

    /// which output to display
    pub show: Show,
}

/// Runs the given commands concurrently, prints their results, and returns the highest exit code.
///
/// # Examples
///
/// ```
/// use conc::{RunArgs, Show, run, shell_executable};
/// use std::process::ExitCode;
///
/// let args = RunArgs {
///     executables: vec![shell_executable("echo one"), shell_executable("echo two")],
///     error_on_output: false,
///     show: Show::All,
/// };
///
/// let exit_code = run(args);
/// assert_eq!(exit_code, ExitCode::SUCCESS);
/// ```
#[must_use]
pub fn run(args: RunArgs) -> ExitCode {
    let (send, receive) = mpsc::channel();

    // execute all commands concurrently and let them signal via the channel when they are done
    for call in args.executables {
        let send_clone = send.clone();
        thread::spawn(move || {
            let _ = send_clone.send(subshell::run(call));
        });
    }

    // drop the original sender so the receiver knows when all senders are closed
    drop(send);

    // print results as they arrive and collect exit codes
    let mut exit_code = 0;
    for call_result in receive {
        match call_result {
            Ok(call_result) => {
                exit_code = exit_code.max(call_result.exit_code());
                let error_from_output = args.error_on_output && call_result.has_output();
                if error_from_output {
                    exit_code = exit_code.max(1);
                }
                let call_failed = !call_result.success() || error_from_output;
                print_result(&call_result, call_failed, args.show);
            }
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                exit_code = exit_code.max(1);
            }
        }
    }
    ExitCode::from(exit_code)
}

/// prints the result of a single command execution to stdout and stderr
fn print_result(call_result: &CallResult, is_failed: bool, show: Show) {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // print command name
    if show.display_command() {
        let mut command = call_result.name.clone();
        if is_failed {
            let _ = writeln!(stdout, "{}", command.bold().red());
        } else {
            if show.display_success() {
                command = command.bold().to_string();
            }
            let _ = writeln!(stdout, "{command}");
        }
    }

    // print command output
    if is_failed || show.display_success() {
        write_output(&mut stdout, &call_result.output.stdout);
        write_output(&mut stderr, &call_result.output.stderr);
    }
}

fn write_output(writer: &mut dyn Write, output: &[u8]) {
    if !output.is_empty() {
        let _ = writer.write_all(output);
        if !output.ends_with(b"\n") {
            let _ = writer.write_all(b"\n");
        }
    }
}
