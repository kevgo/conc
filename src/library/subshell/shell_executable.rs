#[cfg(windows)]
use super::{bash_script, git_bash};
use crate::Executable;
use std::process::Command;

/// Creates an Executable that runs the given command
/// in a shell environment so that shell features can be used.
///
/// In Unix-like environments, this uses the `sh` shell.
/// In Windows, it uses `cmd.exe`.
/// On Windows, a `.sh` or `.bash` file runs through Git Bash
/// when Git for Windows is installed.
#[must_use]
pub fn shell_executable<IS: Into<String>>(command: IS) -> Executable {
    let name = command.into();
    let command = shell_command(&name);
    Executable { name, command }
}

/// Provides a Command instance that executes the given command
/// in a shell environment so that shell features can be used.
///
/// In Unix-like environments, this uses the `sh` shell.
/// In Windows, it uses `cmd.exe`.
#[cfg(unix)]
#[must_use]
pub fn shell_command(command: &str) -> Command {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    cmd
}

/// Provides a Command instance that executes the given command
/// in a shell environment so that shell features can be used.
///
/// Uses `cmd.exe` for ordinary commands.
/// A `.sh` or `.bash` file runs through Git Bash when Git for Windows is installed.
#[cfg(windows)]
#[must_use]
pub fn shell_command(command: &str) -> Command {
    if let Some(bash) = bash_script::bash_command(command, git_bash::find) {
        return bash;
    }
    cmd_command(command)
}

#[cfg(windows)]
fn cmd_command(command: &str) -> Command {
    let mut cmd = Command::new("cmd.exe");
    cmd.arg("/C").arg(command);
    cmd
}

#[cfg(all(test, windows))]
mod tests {
    use super::shell_executable;
    use std::path::Path;
    use std::process::Command;

    fn assert_git_bash(command: &Command) {
        let program = Path::new(command.get_program());
        assert!(
            program
                .file_name()
                .is_some_and(|name| name.eq_ignore_ascii_case("bash.exe")),
            "expected Git Bash, got {}",
            program.display()
        );
    }

    #[test]
    fn runs_bash_script_with_arguments() {
        let dir = tempfile::tempdir().unwrap();
        let script = dir.path().join("hello.sh");
        std::fs::write(&script, "echo \"arg=$1\"\n").unwrap();
        let script_path = script.to_str().unwrap();
        let runnable = format!("\"{script_path}\" \"from bash\"");
        let mut executable = shell_executable(runnable);
        assert_git_bash(&executable.command);
        let output = executable.command.output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("arg=from bash"),
            "stdout: {stdout} stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn returns_script_exit_code() {
        let dir = tempfile::tempdir().unwrap();
        let script = dir.path().join("fail.sh");
        std::fs::write(&script, "exit 4\n").unwrap();
        let script_path = script.to_str().unwrap();
        let mut executable = shell_executable(format!("\"{script_path}\""));
        assert_git_bash(&executable.command);
        let output = executable.command.output().unwrap();
        assert_eq!(output.status.code(), Some(4));
    }
}
