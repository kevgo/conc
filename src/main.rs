mod arguments;
mod cmd;
mod commands;
mod config;
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
    let command = arguments::parse(env::args().skip(1))?;
    match command {
        Command::Help => cmd::help(),
        Command::Run { config, calls } => cmd::run(calls, config),
        Command::Version => cmd::version(),
    }
}
