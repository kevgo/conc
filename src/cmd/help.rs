use std::process::ExitCode;

const HELP: &str = r"
Conc runs commands concurrently and returns the first non-zero exit code it encounters.

Usage: conc [flags] [commands...]

Flags:
  --help, -h           this help text
  --show=[all|failed]  display the output of all or only the failed commands
  --version, -V        display version
";

pub fn help() -> ExitCode {
    println!("{}", &HELP[1..]);
    ExitCode::SUCCESS
}
