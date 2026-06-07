use crate::library::run::Executable;
use std::io;
use std::process::Command;

/// executes the given command
pub fn run(executable: Executable) -> Result<CallResult, RunError> {
    let mut command = executable.command;
    match command.output() {
        Ok(output) => Ok(CallResult {
            name: executable.name,
            output,
        }),
        Err(error) => Err(RunError {
            name: executable.name,
            error,
        }),
    }
}

/// `CallResult` represents the result of a single command execution.
pub struct CallResult {
    pub name: String,
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

/// creates an Executable that runs the given command in a shell
pub fn shell_executable<IS: Into<String>>(command: IS) -> Executable {
    let name = command.into();
    let command = shell_command(&name);
    Executable { name, command }
}

/// provides a Command instance that executes this call in a shell
#[cfg(unix)]
pub fn shell_command(command: &str) -> Command {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    cmd
}

/// provides a Command instance that executes this call in a shell
#[cfg(windows)]
pub fn shell_command(command: &str) -> Command {
    let mut cmd = Command::new("cmd.exe");
    cmd.arg("/C").arg(command);
    cmd
}

fn to_exitcode_u8(value: i32) -> u8 {
    if value == i32::MIN {
        return 255;
    }
    u8::try_from(value.abs()).unwrap_or(255)
}

pub struct RunError {
    /// textual description of the command that failed to execute
    pub name: String,
    /// the error that occurred while executing the command
    pub error: io::Error,
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot execute '{}': {}", self.name, self.error)
    }
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
