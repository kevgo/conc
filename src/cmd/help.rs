use std::process::ExitCode;

const HELP: &str = r"
Conc runs commands concurrently and returns the first non-zero exit code it encounters.

Usage: conc [flags] [commands...]

Flags:
  --help, -h     this help text
  --show=all     show the output of all commands
  --show=failed  show the output of only failed commands
  --version, -V  show the version
";

pub fn help() -> ExitCode {
    println!("{}", &HELP[1..]);
    ExitCode::SUCCESS
}
