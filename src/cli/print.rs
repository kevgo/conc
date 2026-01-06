use std::io::{self, Write};
use std::process::Output;

pub(crate) fn print_success(output: Output) {
    if output.status.success() {
        let mut stdout = io::stdout();
        let _ = stdout.write_all(b"SUCCESS: ");
        let _ = stdout.write_all(&output.stdout);
        let _ = stdout.write_all(b"\n");
    }
}
