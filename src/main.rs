mod binary;

use binary::cli::Command;
use binary::{AppError, cli, cmd};
use colored::Colorize;
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

fn inner() -> Result<ExitCode, AppError> {
    Ok(match cli::parse(env::args().skip(1))? {
        Command::Help => cmd::help(),
        Command::Run {
            calls,
            error_on_output,
            show,
        } => conc::run(conc::RunArgs {
            commands: calls,
            error_on_output,
            show,
        }),
        Command::Version => cmd::version(),
    })
}
