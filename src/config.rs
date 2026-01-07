#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Config {
    pub show: Show,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Show {
    All,
    Failed,
}

impl Show {
    /// indicates whether to display the output of successful commands
    pub(crate) fn display_success(&self) -> bool {
        match self {
            Show::All => true,
            Show::Failed => false,
        }
    }
}
