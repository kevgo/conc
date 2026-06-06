use conc::{Call, ErrorOnOutput, Show};

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
