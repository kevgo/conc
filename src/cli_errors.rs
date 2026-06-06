/// Errors that should be printed to the user to help them use this app correctly.
#[derive(Debug, Eq, PartialEq)]
pub enum UserError {
    UnknownFlag(String),
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::UnknownFlag(flag) => {
                write!(f, "Unknown flag: {flag}")
            }
        }
    }
}

/// A Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point.
pub type Result<T> = core::result::Result<T, UserError>;
