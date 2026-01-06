use crate::errors::UserError;
use crate::subshell::CallResult;
use std::io::{self, Write};

pub(crate) fn output(call_result: &CallResult) {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // Print command name
    let command_name = if call_result.call.arguments.is_empty() {
        format!("[{}]\n", call_result.call.executable)
    } else {
        format!(
            "[{} {}]\n",
            call_result.call.executable,
            call_result.call.arguments.join(" ")
        )
    };
    let _ = stdout.write_all(command_name.as_bytes());

    // Print stdout if not empty
    if !call_result.output.stdout.is_empty() {
        let _ = stdout.write_all(&call_result.output.stdout);
        if !call_result.output.stdout.ends_with(b"\n") {
            let _ = stdout.write_all(b"\n");
        }
    }

    // Print stderr if not empty
    if !call_result.output.stderr.is_empty() {
        let _ = stderr.write_all(&call_result.output.stderr);
        if !call_result.output.stderr.ends_with(b"\n") {
            let _ = stderr.write_all(b"\n");
        }
    }
}

pub(crate) fn user_error(error: &UserError) {
    let mut stderr = io::stderr();
    let _ = stderr.write_all(error.to_string().as_bytes());
}
