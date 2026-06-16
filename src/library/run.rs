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

#[derive(Debug)]
pub enum Runnable {
    /// run a single command
    Single(Executable),

    /// run the given commands one after the other
    Sequence(Vec<Executable>),
}

/// named arguments for the `run` function
#[derive(Debug)]
pub struct RunArgs {
    /// the commands to execute concurrently
    pub runnables: Vec<Runnable>,

    /// whether to error if any command produces output
    pub error_on_output: bool,

    /// whether to redirect stderr output to stdout
    pub stderr_to_stdout: bool,

    /// which output to display
    pub show: Show,
}

/// Runs the given runnables concurrently, prints their results, and returns the highest exit code.
///
/// # Examples
///
/// ```
/// use conc::{Executable, RunArgs, Runnable, Show, run, shell_executable};
/// use std::process::ExitCode;
/// use std::process::Command;
///
/// let mut command = Command::new("echo");
/// command.arg("one");
/// let runnable1 = Runnable::Single(Executable {
///     name: "echo one".into(),
///     command,
/// });
/// let runnable2 = Runnable::Single(shell_executable("echo two"));
/// let runnable3 = Runnable::Sequence(vec![
///     shell_executable("echo three"),
///     shell_executable("echo four"),
/// ]);
/// let args = RunArgs {
///     runnables: vec![runnable1, runnable2, runnable3],
///     error_on_output: false,
///     stderr_to_stdout: false,
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
    for call in args.runnables {
        let send_clone = send.clone();
        thread::spawn(move || {
            subshell::run(call, &send_clone);
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
                print_result(&call_result, call_failed, args.show, args.stderr_to_stdout);
            }
            Err(err) => {
                println!("{}", err.to_string().red());
                exit_code = exit_code.max(1);
            }
        }
    }
    ExitCode::from(exit_code)
}

/// prints the result of a single command execution to stdout and stderr
fn print_result(call_result: &CallResult, is_failed: bool, show: Show, stderr_to_stdout: bool) {
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
        if stderr_to_stdout {
            write_output(&mut stdout, &call_result.output.stderr);
        } else {
            write_output(&mut stderr, &call_result.output.stderr);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell_executable;
    use big_s::S;

    #[test]
    fn single_shell_executable() {
        let exit_code = run(RunArgs {
            runnables: vec![Runnable::Single(shell_executable("echo one"))],
            error_on_output: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::SUCCESS);
    }

    #[test]
    fn single_raw_command() {
        let mut command = Command::new("echo");
        command.arg("one");
        let exit_code = run(RunArgs {
            runnables: vec![Runnable::Single(Executable {
                name: S("echo one"),
                command,
            })],
            error_on_output: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::SUCCESS);
    }

    #[test]
    fn concurrent_commands() {
        let exit_code = run(RunArgs {
            runnables: vec![
                Runnable::Single(shell_executable("echo one")),
                Runnable::Single(shell_executable("echo two")),
                Runnable::Single(shell_executable("echo three")),
            ],
            error_on_output: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::SUCCESS);
    }

    #[test]
    fn sequential_commands() {
        let group = Runnable::Sequence(vec![
            shell_executable("echo one"),
            shell_executable("echo two"),
        ]);
        let exit_code = run(RunArgs {
            runnables: vec![group],
            error_on_output: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::SUCCESS);
    }

    #[test]
    fn sequence_stops_on_failure() {
        let group =
            Runnable::Sequence(vec![shell_executable("exit 2"), shell_executable("exit 3")]);
        let exit_code = run(RunArgs {
            runnables: vec![group],
            error_on_output: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::from(2));
    }

    #[test]
    fn failing_command() {
        let exit_code = run(RunArgs {
            runnables: vec![Runnable::Single(shell_executable("false"))],
            error_on_output: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::FAILURE);
    }

    #[test]
    fn returns_highest_exit_code() {
        let exit_code = run(RunArgs {
            runnables: vec![
                Runnable::Single(shell_executable("exit 0")),
                Runnable::Single(shell_executable("exit 2")),
                Runnable::Single(shell_executable("exit 1")),
            ],
            error_on_output: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::from(2));
    }

    mod error_on_output {
        use big_s::S;

        use crate::{Executable, RunArgs, Runnable, Show, run, shell_executable};
        use std::process::{Command, ExitCode};

        #[test]
        fn outputs_spaces() {
            let mut command = Command::new("echo");
            command.arg("  ");
            let exit_code = run(RunArgs {
                runnables: vec![Runnable::Single(Executable {
                    name: S(""),
                    command,
                })],
                error_on_output: true,
                stderr_to_stdout: false,
                show: Show::All,
            });
            assert_eq!(exit_code, ExitCode::FAILURE);
        }

        #[test]
        fn outputs_nothing() {
            let exit_code = run(RunArgs {
                runnables: vec![Runnable::Single(shell_executable("true"))],
                error_on_output: true,
                stderr_to_stdout: false,
                show: Show::Failed,
            });
            assert_eq!(exit_code, ExitCode::SUCCESS);
        }
    }
}
