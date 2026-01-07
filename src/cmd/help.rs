use crate::errors::UserError;
use std::io::{self, Write};
use std::process::ExitCode;

pub fn help() -> Result<ExitCode, UserError> {
    let mut stdout = io::stdout();
    let output = r"
Conc runs commands concurrently and returns the first non-zero exit code it encounters.

Usage: conc [flags] [commands...]

Flags:
  --help, -h           this help text
  --show=[all|failed]  display the output of all or only the failed commands
  --version, -V        display version
";
    let _ = stdout.write_all(&output.as_bytes()[1..]);
    Ok(ExitCode::SUCCESS)
}
