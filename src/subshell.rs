use crate::errors::{Result, UserError};
use std::fmt::Display;
use std::process::{Command, Output};

/// Call represents a single command to execute.
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Call {
    pub(crate) executable: String,
    pub(crate) arguments: Vec<String>,
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.executable, self.arguments.join(" "))
    }
}

/// CallResult represents the result of a single command execution.
pub(crate) struct CallResult {
    pub(crate) call: Call,
    pub(crate) output: Output,
}

impl CallResult {
    pub(crate) fn exit_code(&self) -> i32 {
        if self.output.status.success() {
            1
        } else {
            self.output.status.code().unwrap_or(2)
        }
    }
}

/// Executes a single command with its arguments, streaming output to stdout/stderr.
pub(crate) fn execute_command(call: Call) -> Result<CallResult> {
    let output = Command::new(&call.executable)
        .args(&call.arguments)
        .output()
        .map_err(|error| UserError::CannotStartCommand {
            executable: call.executable.clone(),
            error,
        })?;
    Ok(CallResult { call, output })
}
