use std::process::ExitCode;

pub fn version() -> ExitCode {
    println!("conc {}", env!("CARGO_PKG_VERSION"));
    ExitCode::SUCCESS
}
