/// the different ways the command output can be displayed
#[derive(Copy, Debug, Eq, PartialEq, Clone)]
pub enum Show {
    /// Display the names of the executed commands, the full command lines, and their output.
    Verbose,

    /// Display the names of the executed commands and their output.
    Output,

    /// Display the names of the executed commands and only the output of failed commands.
    Names,

    /// Display only the names and output of failed commands.
    Failed,
}

impl Show {
    /// indicates whether to display the command name
    #[must_use]
    pub(crate) fn display_name(self) -> bool {
        match self {
            Show::Verbose | Show::Output | Show::Names => true,
            Show::Failed => false,
        }
    }

    /// indicates whether to display the full command line
    #[must_use]
    pub(crate) fn display_command(self) -> bool {
        match self {
            Show::Verbose => true,
            Show::Output | Show::Names | Show::Failed => false,
        }
    }

    /// indicates whether to display the output of successful commands
    #[must_use]
    pub(crate) fn display_success(self) -> bool {
        match self {
            Show::Verbose | Show::Output => true,
            Show::Names | Show::Failed => false,
        }
    }
}
