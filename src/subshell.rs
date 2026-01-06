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

#[cfg(test)]
mod tests {

    #[test]
    fn test_execute_command_success() {
        let cmd_args = strs(&["echo", "test"]);
        let result = execute_command(cmd_args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_execute_command_with_exit_code() {
        let cmd_args = strs(&["sh", "-c", "exit 42"]);
        let result = execute_command(cmd_args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_execute_command_empty_args() {
        let cmd_args: Vec<String> = Vec::new();
        let result = execute_command(cmd_args);
        assert!(result.is_err());
        assert!(matches!(result, Err(CommandError::EmptyCommand)));
    }

    #[test]
    fn test_execute_command_nonexistent() {
        let cmd_args = strs(&["nonexistent_command_xyz123"]);
        let result = execute_command(cmd_args);
        assert!(result.is_err());
        assert!(matches!(result, Err(CommandError::SpawnFailed(_, _))));
    }
}
