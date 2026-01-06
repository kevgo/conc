use crate::cli::arguments::Call;
use crate::errors::{Result, UserError};
use std::process::{Command, Output};

// It runs each command in a thread and captures the output.
// When the command is done, it returns the output and exit code.

/// Executes a single command with its arguments, streaming output to stdout/stderr.
pub(crate) fn execute_command(call: Call) -> Result<Output> {
    Command::new(&call.executable)
        .args(&call.arguments)
        .output()
        .map_err(|error| UserError::CannotStartCommand {
            executable: call.executable,
            error,
        })
}
