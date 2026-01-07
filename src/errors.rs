use crate::subshell::Call;

/// errors that should be printed to the user to help them use this app correctly
#[derive(Debug, PartialEq)]
pub(crate) enum UserError {
    CannotStartCommand { call: Call, error: String },
    NoCommandsProvided,
    UnknownFlag(String),
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::CannotStartCommand { call, error } => {
                write!(f, "Cannot start command '{call}': {error}")
            }
            UserError::NoCommandsProvided => {
                write!(f, "No commands provided")
            }
            UserError::UnknownFlag(flag) => {
                write!(f, "Unknown flag: {flag}")
            }
        }
    }
}
