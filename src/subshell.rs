use crate::errors::UserError;
use std::process::Command;

/// Call represents a command to execute.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Call(String);

impl Call {
    /// provides a Command instance to execute this call in a Unix shell
    #[cfg(unix)]
    pub(crate) fn command(&self) -> Command {
        let mut command = Command::new("sh");
        command.arg("-c").arg(&self.0);
        command
    }

    /// provides a Command instance to execute this call in a Windows shell
    #[cfg(windows)]
    pub(crate) fn command(&self) -> Command {
        let mut command = Command::new("cmd.exe");
        command.arg("/C").arg(&self.0);
        command
    }

    /// Executes a single command with its arguments, streaming output to stdout/stderr.
    pub(crate) fn run(self) -> Result<CallResult, UserError> {
        let mut command = self.command();
        let output = command.output().map_err(|err| UserError::CannotRunCall {
            call: self.clone(),
            error: err.to_string(),
        })?;
        Ok(CallResult { call: self, output })
    }
}

impl std::fmt::Display for Call {
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
        Call(value.to_string())
    }
}

/// CallResult represents the result of a single command execution.
pub(crate) struct CallResult {
    pub call: Call,
    pub output: std::process::Output,
}

impl CallResult {
    pub(crate) fn exit_code(&self) -> i32 {
        if self.output.status.success() {
            0
        } else {
            self.output.status.code().unwrap_or(1)
        }
    }
}
