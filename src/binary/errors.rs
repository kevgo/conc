/// Errors that should be printed to the user to help them use this app correctly.
#[derive(Debug, Eq, PartialEq)]
pub enum CliError {
    UnknownFlag(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::UnknownFlag(flag) => {
                write!(f, "Unknown flag: {flag}")
            }
        }
    }
}
