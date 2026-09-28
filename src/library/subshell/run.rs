use super::run_error::RunError;
use crate::{Executable, Sequence};
use std::sync::mpsc::Sender;

/// Runs the given sequence of executables and communicates the results through the given MPSC sender.
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

/// Runs the given executable
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

/// represents the result of running an `Executable`
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

fn to_exitcode_u8(value: i32) -> u8 {
    if value == i32::MIN {
        return 255;
    }
    u8::try_from(value.abs()).unwrap_or(255)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_convert_to_u8() {
        assert_eq!(to_exitcode_u8(0), 0);
        assert_eq!(to_exitcode_u8(1), 1);
        assert_eq!(to_exitcode_u8(-1), 1);
        assert_eq!(to_exitcode_u8(255), 255);
        assert_eq!(to_exitcode_u8(-255), 255);
        assert_eq!(to_exitcode_u8(256), 255);
        assert_eq!(to_exitcode_u8(-256), 255);
        assert_eq!(to_exitcode_u8(i32::MAX), 255);
        assert_eq!(to_exitcode_u8(i32::MIN), 255);
    }
}
