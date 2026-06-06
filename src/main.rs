mod binary;
mod cli;
mod cmd;
mod commands;

use binary::CliError;
use colored::Colorize;
use commands::Command;
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

fn inner() -> Result<ExitCode, CliError> {
    Ok(match cli::parse(env::args().skip(1))? {
        Command::Help => cmd::help(),
        Command::Run {
            calls,
            error_on_output,
            show,
        } => conc::run(calls, error_on_output, show),
        Command::Version => cmd::version(),
    })
}
