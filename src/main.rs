mod cli;
mod cmd;
mod commands;
mod errors;
mod subshell;

use colored::Colorize;
use commands::Command;
use errors::Result;
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

<<<<<<< Updated upstream
fn inner() -> Result<ExitCode, UserError> {
    Ok(match cli::parse(env::args().skip(1))? {
=======
fn inner() -> Result<ExitCode> {
    Ok(match arguments::parse(env::args().skip(1))? {
>>>>>>> Stashed changes
        Command::Help => cmd::help(),
        Command::Run { calls, show } => cmd::run(calls, show),
        Command::Version => cmd::version(),
    })
}
