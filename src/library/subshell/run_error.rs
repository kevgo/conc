use std::io;

pub struct RunError {
    /// display version of the command that failed to execute
    pub name: String,
    /// the error that occurred while executing the command
    pub error: io::Error,
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot execute '{}': {}", self.name, self.error)
    }
}
