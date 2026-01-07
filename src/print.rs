use crate::config::Show;
use crate::errors::UserError;
use crate::subshell::CallResult;
use colored::Colorize;
use std::io::{self, Write};

/// prints the result of a single command execution to stdout and stderr
pub fn result(call_result: &CallResult, show: &Show) {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // print command name
    if call_result.output.status.success() {
        let mut command = call_result.call.to_string();
        if show.display_success() {
            command = command.bold().to_string();
        }
        let _ = writeln!(stdout, "{command}");
    } else {
        let command = call_result.call.to_string().bold().red();
        let _ = writeln!(stdout, "{command}");
    }

    if call_result.output.status.success() && !show.display_success() {
        return;
    }

    write_output(&mut stdout, &call_result.output.stdout);
    write_output(&mut stderr, &call_result.output.stderr);
}

fn write_output(writer: &mut dyn Write, output: &[u8]) {
    if !output.is_empty() {
        let _ = writer.write_all(output);
        if !output.ends_with(b"\n") {
            let _ = writer.write_all(b"\n");
        }
    }
}

/// prints the given user error to stderr
pub fn error(error: &UserError) {
    let mut stderr = io::stderr();
    let _ = stderr.write_all(error.to_string().as_bytes());
}

pub fn help() {
    let mut stdout = io::stdout();
    let output = r#"
Conc runs commands concurrently and returns the first non-zero exit code it encounters.

Usage: conc [flags] [commands...]

Flags:
  --help, -h           this help text
  --show=[all|failed]  display the output of all or only the failed commands
  --version, -V        display version
"#;
    let _ = stdout.write_all(output[1..].as_bytes());
}

pub fn version() {
    let mut stdout = io::stdout();
    let _ = writeln!(stdout, "conc {}", env!("CARGO_PKG_VERSION"));
}
