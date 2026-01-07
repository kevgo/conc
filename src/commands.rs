use crate::subshell::Call;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Help,
    Run { calls: Vec<Call>, show: Show },
    Version,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Show {
    /// display the output of all commands
    All,

    /// display only the output of failed commands
    Failed,
}

impl Show {
    /// indicates whether to display the output of successful commands
    pub fn display_success(&self) -> bool {
        match self {
            Show::All => true,
            Show::Failed => false,
        }
    }
}
