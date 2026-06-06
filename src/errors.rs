use crate::subshell::Call;

/// Errors that can occur while running commands.
#[derive(Debug, Eq, PartialEq)]
pub enum UserError {
    CannotRunCall { call: Call, error: String },
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::CannotRunCall { call, error } => {
                write!(f, "Cannot start command '{call}': {error}")
            }
        }
    }
}

/// A Result that always has a `UserError` as the error.
pub type Result<T> = core::result::Result<T, UserError>;
