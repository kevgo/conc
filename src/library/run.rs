use super::Show;
use crate::Executable;
use crate::library::subshell;
use colored::Colorize;
use nonempty::NonEmpty;
use std::fmt::Debug;
use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;

/// named arguments for the `run` function
#[derive(Debug)]
pub struct RunArgs {
    /// the commands to execute
    pub sequences: Vec<NonEmpty<Executable>>,

    /// whether to error if any command produces output
    pub error_on_output: bool,

    /// whether to execute the sequences one after the other instead of concurrently
    pub sequential: bool,

    /// whether to redirect stderr output to stdout
    pub stderr_to_stdout: bool,

    /// which output to display
    pub show: Show,
}

/// Runs the given runnables, prints their results, and returns the highest exit code.
///
/// Runs them concurrently by default. If `RunArgs::sequential` is enabled,
/// runs them one after the other in the given order.
/// A failing sequence does not prevent the sequences after it from running.
///
/// # Examples
///
/// ```
/// use conc::{Executable, RunArgs, Sequence, Show, run, shell_executable};
/// use std::process::ExitCode;
/// use std::process::Command;
///
/// let mut command = Command::new("echo");
/// command.arg("one");
/// let sequence1 = Sequence::one(Executable {
///     name: "echo one".into(),
///     command,
/// });
/// let sequence2 = Sequence::one(shell_executable("echo two"));
/// let sequence3 = Sequence::many(shell_executable("echo three"), vec![shell_executable("echo four")]);
/// let args = RunArgs {
///     sequences: vec![sequence1, sequence2, sequence3],
///     error_on_output: false,
///     sequential: false,
///     stderr_to_stdout: false,
///     show: Show::Output,
/// };
///
/// let exit_code = run(args);
/// assert_eq!(exit_code, ExitCode::SUCCESS);
/// ```
#[must_use]
pub fn run(args: RunArgs) -> ExitCode {
    let (send, receive) = mpsc::channel();

    if args.sequential {
        // execute all commands one after the other and let them signal via the channel when they are done,
        // the sender gets dropped when the thread ends so the receiver knows when all commands are done
        thread::spawn(move || {
            for call in args.sequences {
                subshell::run(call, &send, args.error_on_output);
            }
        });
    } else {
        // execute all commands concurrently and let them signal via the channel when they are done
        for call in args.sequences {
            let send_clone = send.clone();
            thread::spawn(move || {
                subshell::run(call, &send_clone, args.error_on_output);
            });
        }

        // drop the original sender so the receiver knows when all senders are closed
        drop(send);
    }

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
fn print_result(
    call_result: &subshell::CallResult,
    is_failed: bool,
    show: Show,
    stderr_to_stdout: bool,
) {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // print command name
    if show.display_name() {
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

    // print full command line
    if show.display_command() {
        let _ = writeln!(stdout, "{}", call_result.command_line);
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
    use crate::Executable;
    use crate::shell_executable;
    use big_s::S;
    use nonempty::nonempty;
    use std::process::Command;

    #[test]
    fn single_shell_executable_verbose() {
        let exit_code = run(RunArgs {
            sequences: vec![nonempty![shell_executable("echo one")]],
            error_on_output: false,
            sequential: false,
            stderr_to_stdout: false,
            show: Show::Verbose,
        });
        assert_eq!(exit_code, ExitCode::SUCCESS);
    }

    #[test]
    fn single_shell_executable() {
        let exit_code = run(RunArgs {
            sequences: vec![nonempty![shell_executable("echo one")]],
            error_on_output: false,
            sequential: false,
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
            sequences: vec![nonempty![Executable {
                name: S("echo one"),
                command,
            }]],
            error_on_output: false,
            sequential: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::SUCCESS);
    }

    #[test]
    fn concurrent_commands() {
        let exit_code = run(RunArgs {
            sequences: vec![
                nonempty![shell_executable("echo one")],
                nonempty![shell_executable("echo two")],
                nonempty![shell_executable("echo three")],
            ],
            error_on_output: false,
            sequential: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::SUCCESS);
    }

    #[test]
    fn sequential_commands() {
        let group = nonempty![shell_executable("echo one"), shell_executable("echo two")];
        let exit_code = run(RunArgs {
            sequences: vec![group],
            error_on_output: false,
            sequential: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::SUCCESS);
    }

    #[test]
    fn sequence_stops_on_failure() {
        let group = nonempty![shell_executable("exit 2"), shell_executable("exit 3")];
        let exit_code = run(RunArgs {
            sequences: vec![group],
            error_on_output: false,
            sequential: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::from(2));
    }

    #[test]
    fn failing_command() {
        let exit_code = run(RunArgs {
            sequences: vec![nonempty![shell_executable("false")]],
            error_on_output: false,
            sequential: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::FAILURE);
    }

    #[test]
    fn returns_highest_exit_code() {
        let exit_code = run(RunArgs {
            sequences: vec![
                nonempty![shell_executable("exit 0")],
                nonempty![shell_executable("exit 2")],
                nonempty![shell_executable("exit 1")],
            ],
            error_on_output: false,
            sequential: false,
            stderr_to_stdout: false,
            show: Show::Failed,
        });
        assert_eq!(exit_code, ExitCode::from(2));
    }

    mod sequential {
        use nonempty::nonempty;

        use crate::{RunArgs, Show, run, shell_executable};
        use std::fs;
        use std::process::ExitCode;

        #[test]
        fn runs_in_the_given_order() {
            let dir = tempfile::tempdir().unwrap();
            let mut slow = shell_executable("sleep 0.2 && echo one >> log");
            slow.command.current_dir(&dir);
            let mut fast = shell_executable("echo two >> log");
            fast.command.current_dir(&dir);
            let exit_code = run(RunArgs {
                sequences: vec![nonempty![slow], nonempty![fast]],
                error_on_output: false,
                sequential: true,
                stderr_to_stdout: false,
                show: Show::Failed,
            });
            assert_eq!(exit_code, ExitCode::SUCCESS);
            let have = fs::read_to_string(dir.path().join("log")).unwrap();
            assert_eq!(have, "one\ntwo\n");
        }

        #[test]
        fn continues_after_failure() {
            let dir = tempfile::tempdir().unwrap();
            let mut succeeding = shell_executable("echo two >> log");
            succeeding.command.current_dir(&dir);
            let exit_code = run(RunArgs {
                sequences: vec![nonempty![shell_executable("exit 2")], nonempty![succeeding]],
                error_on_output: false,
                sequential: true,
                stderr_to_stdout: false,
                show: Show::Failed,
            });
            assert_eq!(exit_code, ExitCode::from(2));
            let have = fs::read_to_string(dir.path().join("log")).unwrap();
            assert_eq!(have, "two\n");
        }
    }

    mod error_on_output {
        use crate::{Executable, RunArgs, Show, run, shell_executable};
        use big_s::S;
        use nonempty::nonempty;
        use std::process::{Command, ExitCode};

        #[test]
        fn outputs_spaces() {
            let mut command = Command::new("echo");
            command.arg("  ");
            let exit_code = run(RunArgs {
                sequences: vec![nonempty![Executable {
                    name: S(""),
                    command,
                }]],
                error_on_output: true,
                sequential: false,
                stderr_to_stdout: false,
                show: Show::Output,
            });
            assert_eq!(exit_code, ExitCode::FAILURE);
        }

        #[test]
        fn outputs_nothing() {
            let exit_code = run(RunArgs {
                sequences: vec![nonempty![shell_executable("true")]],
                error_on_output: true,
                sequential: false,
                stderr_to_stdout: false,
                show: Show::Failed,
            });
            assert_eq!(exit_code, ExitCode::SUCCESS);
        }

        #[test]
        fn sequence_stops_when_step_has_output() {
            let mut command = Command::new("echo");
            command.arg("  ");
            let exit_code = run(RunArgs {
                sequences: vec![nonempty![
                    Executable {
                        name: S(""),
                        command,
                    },
                    shell_executable("exit 3"),
                ]],
                error_on_output: true,
                sequential: false,
                stderr_to_stdout: false,
                show: Show::Failed,
            });
            assert_eq!(exit_code, ExitCode::from(1));
        }
    }
}
