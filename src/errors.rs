use std::io;

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub(crate) type Result<T> = core::result::Result<T, UserError>;

/// errors that should be printed to the user to help them use this app correctly
#[derive(Debug)]
pub(crate) enum UserError {
    CannotStartCommand {
        executable: String,
        error: io::Error,
    },
    NoCommandsProvided,
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::CannotStartCommand { executable, error } => {
                write!(f, "Cannot start command '{executable}': {error}")
            }
            UserError::NoCommandsProvided => {
                write!(f, "No commands provided")
            }
        }
    }
}
