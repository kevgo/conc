use crate::config::Show;
use crate::errors::UserError;
use crate::subshell::CallResult;
use colored::Colorize;
use std::io::{self, Write};

/// prints the result of a single command execution to stdout and stderr
pub(crate) fn result(call_result: &CallResult, show: &Show) {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // print command name
    if call_result.output.status.success() {
        let command = call_result.call.to_string().bold();
        let _ = stdout.write_all(format!("{}\n", command).as_bytes());
    } else {
        let command = call_result.call.to_string().bold().red();
        let _ = stdout.write_all(format!("{}\n", command).as_bytes());
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
pub(crate) fn error(error: &UserError) {
    let mut stderr = io::stderr();
    let _ = stderr.write_all(error.to_string().as_bytes());
}
