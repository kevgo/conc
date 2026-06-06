/// the different ways the command output can be displayed
#[derive(Copy, Debug, Eq, PartialEq, Clone)]
pub enum Show {
    /// Display the names of the executed commands and their output.
    All,

    /// Display the names of the executed commands and only the output of failed commands.
    Names,

    /// Display only the names and output of failed commands.
    Failed,
}

impl Show {
    /// indicates whether to display the command name
    #[must_use]
    pub(crate) fn display_command(self) -> bool {
        match self {
            Show::All | Show::Names => true,
            Show::Failed => false,
        }
    }

    /// indicates whether to display the output of successful commands
    #[must_use]
    pub(crate) fn display_success(self) -> bool {
        match self {
            Show::All => true,
            Show::Names | Show::Failed => false,
        }
    }
}
