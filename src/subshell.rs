use crate::errors::UserError;
use std::fmt::Display;
use std::process::Command;

/// Call represents a single command to execute.
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Call {
    pub executable: String,
    pub arguments: Vec<String>,
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.executable, self.arguments.join(" "))
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
    let output = Command::new(&call.executable)
        .args(&call.arguments)
        .output()
        .map_err(|error| UserError::CannotStartCommand {
            executable: call.executable.clone(),
            error,
        })?;
    Ok(CallResult { call, output })
}
