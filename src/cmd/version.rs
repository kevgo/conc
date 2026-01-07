use std::io::{self, Write};
use std::process::ExitCode;

use crate::errors::UserError;

pub fn version() -> Result<ExitCode, UserError> {
    let mut stdout = io::stdout();
    let _ = writeln!(stdout, "conc {}", env!("CARGO_PKG_VERSION"));
    Ok(ExitCode::SUCCESS)
}
