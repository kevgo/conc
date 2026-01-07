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
        if self.arguments.is_empty() {
            f.write_str(&self.executable)
        } else {
            write!(f, "{} {}", self.executable, self.arguments.join(" "))
        }
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
    // Build the command string from executable and arguments
    let command_string = if call.arguments.is_empty() {
        call.executable.clone()
    } else {
        format!("{} {}", call.executable, call.arguments.join(" "))
    };

    // Execute the command inside a shell
    let output = if cfg!(unix) {
        // On Unix, use bash
        Command::new("sh").arg("-c").arg(&command_string).output()
    } else if cfg!(windows) {
        // On Windows, use cmd.exe
        Command::new("cmd.exe")
            .arg("/C")
            .arg(&command_string)
            .output()
    } else {
        // Fallback: try to execute directly (shouldn't happen in practice)
        Command::new(&call.executable)
            .args(&call.arguments)
            .output()
    };

    match output {
        Ok(output) => Ok(CallResult { call, output }),
        Err(error) => Err(UserError::CannotStartCommand {
            call,
            error: error.to_string(),
        }),
    }
}
