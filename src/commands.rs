use crate::subshell::Call;

/// the different top-level commands that conc can execute
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    /// display the help text
    Help,
    /// execute the given commands concurrently
    Run {
        calls: Vec<Call>,
        error_on_output: ErrorOnOutput,
        show: Show,
    },
    /// display the version
    Version,
}

/// whether to error if any command produces output
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ErrorOnOutput(bool);

impl From<bool> for ErrorOnOutput {
    fn from(value: bool) -> Self {
        ErrorOnOutput(value)
    }
}

impl From<ErrorOnOutput> for bool {
    fn from(value: ErrorOnOutput) -> Self {
        value.0
    }
}

/// the different ways to display the output of the commands
#[derive(Copy, Debug, Eq, PartialEq, Clone)]
pub enum Show {
    /// display the output of all commands
    All,

    /// display only the output of failed commands
    Failed,
}

impl Show {
    /// indicates whether to display the output of successful commands
    pub fn display_success(self) -> bool {
        match self {
            Show::All => true,
            Show::Failed => false,
        }
    }
}
