/// The different ways to display the output of the commands.
#[derive(Copy, Debug, Eq, PartialEq, Clone)]
pub enum Show {
    /// Display all executed commands and their output.
    All,

    /// Display the names of all executed commands and the output of failed commands.
    Names,

    /// Display only failed commands.
    Failed,
}

impl Show {
    /// indicates whether to display the command name
    #[must_use]
    pub fn display_command(self) -> bool {
        match self {
            Show::All | Show::Names => true,
            Show::Failed => false,
        }
    }

    /// indicates whether to display the output of successful commands
    #[must_use]
    pub fn display_success(self) -> bool {
        match self {
            Show::All => true,
            Show::Names | Show::Failed => false,
        }
    }
}
