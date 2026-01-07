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

    if !call_result.output.stdout.is_empty() {
        let _ = stdout.write_all(&call_result.output.stdout);
        if !call_result.output.stdout.ends_with(b"\n") {
            let _ = stdout.write_all(b"\n");
        }
    }
    if !call_result.output.stderr.is_empty() {
        let _ = stderr.write_all(&call_result.output.stderr);
        if !call_result.output.stderr.ends_with(b"\n") {
            let _ = stderr.write_all(b"\n");
        }
    }
}

/// prints the given user error to stderr
pub(crate) fn error(error: &UserError) {
    let mut stderr = io::stderr();
    let _ = stderr.write_all(error.to_string().as_bytes());
}
