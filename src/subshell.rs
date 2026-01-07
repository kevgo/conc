use crate::errors::UserError;
use std::fmt::Display;
use std::process::Command;

/// Call represents a command to execute.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Call(String);

impl Call {
    /// provides a Command instance that executes this call in a shell
    #[cfg(unix)]
    pub(crate) fn command(&self) -> Command {
        let mut command = Command::new("sh");
        command.arg("-c").arg(&self.0);
        command
    }

    /// provides a Command instance that executes this call in a shell
    #[cfg(windows)]
    pub(crate) fn command(&self) -> Command {
        let mut command = Command::new("cmd.exe");
        command.arg("/C").arg(&self.0);
        command
    }

    /// Executes this call in a shell
    pub(crate) fn run(self) -> Result<CallResult, UserError> {
        let mut command = self.command();
        let output = command.output().map_err(|err| UserError::CannotRunCall {
            call: self.clone(),
            error: err.to_string(),
        })?;
        Ok(CallResult { call: self, output })
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for Call {
    fn from(value: String) -> Self {
        Call(value)
    }
}

impl From<&str> for Call {
    fn from(value: &str) -> Self {
        Call(value.to_owned())
    }
}

/// `CallResult` represents the result of a single command execution.
pub struct CallResult {
    pub call: Call,
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
}

fn to_exitcode_u8(value: i32) -> u8 {
    if value == i32::MIN {
        return 255;
    }
    #[allow(clippy::cast_possible_truncation)] // we reduce the value to 255 before casting
    #[allow(clippy::cast_sign_loss)] // we get the absolute value before casting
    u8::try_from(value.abs().min(255)).unwrap_or(255)
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
