use std::io;

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub(crate) type Result<T> = core::result::Result<T, UserError>;

/// Represents errors that can occur during command execution.
#[derive(Debug)]
pub(crate) enum UserError {
    CannotStartCommand {
        executable: String,
        error: io::Error,
    },
    WaitFailed(io::Error),
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::CannotStartCommand { executable, error } => {
                write!(f, "Cannot start command '{}': {}", executable, error)
            }
            UserError::WaitFailed(err) => write!(f, "Failed to wait for process: {}", err),
        }
    }
}

impl std::error::Error for UserError {}
