use super::exit_code::to_exitcode_u8;
use super::run_error::RunError;
use crate::{Executable, Sequence};
use std::sync::mpsc::Sender;

pub fn run(
    runnable: Sequence,
    sender: &Sender<Result<CallResult, RunError>>,
    error_on_output: bool,
) {
    for executable in runnable {
        let result = execute(executable);
        let failed = match &result {
            Ok(call_result) => {
                !call_result.output.status.success()
                    || (error_on_output && call_result.has_output())
            }
            Err(_) => true,
        };
        let _ = sender.send(result);
        if failed {
            break;
        }
    }
}

fn execute(executable: Executable) -> Result<CallResult, RunError> {
    let command_line = executable.command_line();
    let Executable { mut command, name } = executable;
    match command.output() {
        Ok(output) => Ok(CallResult {
            name,
            command_line,
            output,
        }),
        Err(error) => Err(RunError { name, error }),
    }
}

/// `CallResult` represents the result of a single command execution.
pub struct CallResult {
    pub name: String,
    pub command_line: String,
    pub output: std::process::Output,
}

impl CallResult {
    pub(crate) fn exit_code(&self) -> u8 {
        if self.output.status.success() {
            0
        } else {
            to_exitcode_u8(self.output.status.code().unwrap_or(1))
        }
    }

    /// indicates whether this call produced any output to STDOUT or STDERR
    pub(crate) fn has_output(&self) -> bool {
        !self.output.stdout.is_empty() || !self.output.stderr.is_empty()
    }

    /// indicates whether this call exited with a success code
    pub(crate) fn success(&self) -> bool {
        self.output.status.success()
    }
}
