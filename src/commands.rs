use crate::subshell::Call;

/// the different top-level commands that conc can execute
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    /// display the help text
    Help,
    /// execute the given commands concurrently
    Run { calls: Vec<Call>, show: Show },
    /// display the version
    Version,
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
