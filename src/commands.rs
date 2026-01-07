use crate::config::Config;
use crate::subshell::Call;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Help,
    Run { config: Config, calls: Vec<Call> },
    Version,
}
