/// Errors that originating in the application part
/// that should be printed to the user
/// to help them use this app correctly.
#[derive(Debug, Eq, PartialEq)]
pub enum AppError {
    UnknownFlag(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::UnknownFlag(flag) => {
                write!(f, "Unknown flag: {flag}")
            }
        }
    }
}
