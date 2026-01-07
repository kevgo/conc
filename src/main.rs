mod arguments;
mod cmd;
mod commands;
mod errors;
mod subshell;

use crate::commands::Command;
use colored::Colorize;
use errors::UserError;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            eprintln!("{}", err.to_string().red());
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<ExitCode, UserError> {
    Ok(match arguments::parse(env::args().skip(1))? {
        Command::Help => cmd::help(),
        Command::Run { calls, show } => cmd::run(calls, &show),
        Command::Version => cmd::version(),
    })
}
