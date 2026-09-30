use crate::Executable;
use std::process::Command;

/// Creates an Executable that runs the given command
/// in a shell environment so that shell features can be used.
///
/// In Unix-like environments, this uses the `sh` shell.
/// In Windows, it uses `cmd.exe`.
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
/// In Unix-like environments, this uses the `sh` shell.
/// In Windows, it uses `cmd.exe`.
#[cfg(windows)]
#[must_use]
pub fn shell_command(command: &str) -> Command {
    let mut cmd = Command::new("bash.exe");
    cmd.arg("-c").arg(command);
    cmd
}
