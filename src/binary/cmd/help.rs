use std::process::ExitCode;

const HELP: &str = r#"
Conc runs commands concurrently and returns the first non-zero exit code it encounters.

Usage: conc [flags] [commands...]

Flags:
  --error-on-output   error if any command produces output
  --help, -h          this help text
  --show=all          show the name and output of all commands
  --show=names        show the names of all commands
  --show=failed       show only the output of failed commands
  --stderr-to-stdout  print stderr of commands to stdout
  --version, -V       show the version

Examples:

conc --show=failed "echo one" "echo two" "echo three"

This executes the following commands concurrently:

- echo one
- echo two
- echo three

If any of the commands exit with a non-zero exit code,
conc will print the output of the failed command
and return that exit code.
"#;

pub fn help() -> ExitCode {
    println!("{}", &HELP[1..]);
    ExitCode::SUCCESS
}
