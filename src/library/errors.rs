use crate::library::Call;

/// Errors that can occur while running commands.
#[derive(Debug, Eq, PartialEq)]
pub enum ConcError {
    CannotExecute { call: Call, error: String },
}

impl std::fmt::Display for ConcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConcError::CannotExecute { call, error } => {
                write!(f, "Cannot execute '{call}': {error}")
            }
        }
    }
}
