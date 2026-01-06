use crate::cli::arguments::Call;
use crate::errors::UserError;
use std::io::{self, Write};
use std::process::Output;

pub(crate) fn output(call: &Call, output: &Output) {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // Print command name
    let command_name = if call.arguments.is_empty() {
        format!("[{}]\n", call.executable)
    } else {
        format!("[{} {}]\n", call.executable, call.arguments.join(" "))
    };
    let _ = stdout.write_all(command_name.as_bytes());

    // Print stdout if not empty
    if !output.stdout.is_empty() {
        let _ = stdout.write_all(&output.stdout);
        if !output.stdout.ends_with(b"\n") {
            let _ = stdout.write_all(b"\n");
        }
    }

    // Print stderr if not empty
    if !output.stderr.is_empty() {
        let _ = stderr.write_all(&output.stderr);
        if !output.stderr.ends_with(b"\n") {
            let _ = stderr.write_all(b"\n");
        }
    }
}

pub(crate) fn error(command: &Call, error: &UserError) {
    let mut stderr = io::stderr();
    let _ =
        stderr.write_all(format!("Error executing command '{command}': {}\n", error).as_bytes());
}
