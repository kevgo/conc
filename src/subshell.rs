use crate::errors::UserError;
use std::process::Command;

/// Call represents a single command to execute.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Call(String);

impl Call {
    #[cfg(unix)]
    pub(crate) fn command(&self) -> Command {
        let mut command = Command::new("sh");
        command.arg("-c").arg(&self.0);
        command
    }

    #[cfg(windows)]
    pub(crate) fn command(&self) -> Command {
        let mut command = Command::new("cmd.exe");
        command.arg("/C").arg(&self.0);
        command
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

/// Executes a single command with its arguments, streaming output to stdout/stderr.
pub(crate) fn execute_command(call: Call) -> Result<CallResult, UserError> {
    let mut command = call.command();
    let output = command
        .output()
        .map_err(|err| UserError::CannotStartCommand {
            call: call.clone(),
            error: err.to_string(),
        })?;
    Ok(CallResult { call, output })
}
