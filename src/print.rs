use crate::errors::UserError;
use crate::subshell::CallResult;
use std::io::{self, Write};

/// prints the result of a single command execution to stdout and stderr
pub(crate) fn result(call_result: &CallResult) {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // print command name
    let _ = stdout.write_all(format!("[{}]\n", call_result.call).as_bytes());

    // print stdout if not empty
    if !call_result.output.stdout.is_empty() {
        let _ = stdout.write_all(&call_result.output.stdout);
        if !call_result.output.stdout.ends_with(b"\n") {
            let _ = stdout.write_all(b"\n");
        }
    }

    // print stderr if not empty
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
