//! Decides when a Windows command should run through Git Bash.
//!
//! A command runs through Git Bash when it invokes a `.sh` or `.bash` file,
//! optionally followed by arguments.
//! Shell expressions such as `script.sh && echo done` keep using `cmd.exe`.

use std::path::{Path, PathBuf};
use std::process::Command;

const SHELL_METACHARACTERS: &[char] = &['&', '|', ';', '<', '>', '(', ')', '`', '\n', '\r'];

/// Builds a Git Bash command for `runnable` when it is a Bash script.
///
/// `git_bash` is called only when the runnable is a script file.
#[must_use]
pub(super) fn bash_command<F>(runnable: &str, git_bash: F) -> Option<Command>
where
    F: FnOnce() -> Option<PathBuf>,
{
    let (program, args) = script_invocation(runnable)?;
    let bash = git_bash()?;
    let mut command = Command::new(bash);
    command.arg(program);
    command.args(args);
    Some(command)
}

fn script_invocation(runnable: &str) -> Option<(String, Vec<String>)> {
    let runnable = runnable.trim();
    if runnable.is_empty() {
        return None;
    }
    if !has_unquoted_shell_syntax(runnable) {
        if let Some(parts) = split_command(runnable) {
            let mut parts = parts.into_iter();
            if let Some(program) = parts.next() {
                if has_bash_extension(&program) {
                    return Some((program, parts.collect()));
                }
            }
        }
    }
    existing_script_path(runnable)
}

/// Uses the whole command as a script path when that path exists.
///
/// Word splitting would break an unquoted path that contains spaces.
fn existing_script_path(runnable: &str) -> Option<(String, Vec<String>)> {
    let unquoted = strip_matching_quotes(runnable);
    if has_dir_separator(unquoted) && has_bash_extension(unquoted) && Path::new(unquoted).is_file()
    {
        return Some((unquoted.to_owned(), Vec::new()));
    }
    None
}

fn has_bash_extension(program: &str) -> bool {
    Path::new(program)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("sh") || extension.eq_ignore_ascii_case("bash")
        })
}

fn has_dir_separator(path: &str) -> bool {
    path.contains(['/', '\\'])
}

fn has_unquoted_shell_syntax(command: &str) -> bool {
    let mut quote = None;
    for character in command.chars() {
        if let Some(open) = quote {
            if character == open {
                quote = None;
            }
            continue;
        }
        if character == '"' || character == '\'' {
            quote = Some(character);
            continue;
        }
        if SHELL_METACHARACTERS.contains(&character) {
            return true;
        }
    }
    false
}

/// Splits a command into words.
///
/// Quotes group words. Backslashes stay literal so Windows paths are preserved.
/// Returns `None` when a quote is not closed.
fn split_command(command: &str) -> Option<Vec<String>> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    let mut token_started = false;
    for character in command.chars() {
        if let Some(open) = quote {
            if character == open {
                quote = None;
            } else {
                current.push(character);
            }
            token_started = true;
            continue;
        }
        if character == '"' || character == '\'' {
            quote = Some(character);
            token_started = true;
            continue;
        }
        if character.is_whitespace() {
            if token_started {
                parts.push(std::mem::take(&mut current));
                token_started = false;
            }
            continue;
        }
        current.push(character);
        token_started = true;
    }
    if quote.is_some() {
        return None;
    }
    if token_started {
        parts.push(current);
    }
    if parts.is_empty() { None } else { Some(parts) }
}

fn strip_matching_quotes(input: &str) -> &str {
    let bytes = input.as_bytes();
    if input.len() >= 2 {
        let first = bytes[0];
        let last = bytes[input.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return &input[1..input.len() - 1];
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn bash(runnable: &str) -> Option<Command> {
        bash_command(runnable, || Some(PathBuf::from("bash.exe")))
    }

    fn args(command: &Command) -> Vec<String> {
        command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn sh_file() {
        let command = bash("demo.sh").unwrap();
        assert_eq!(command.get_program(), "bash.exe");
        let have = args(&command);
        let want = vec!["demo.sh".to_owned()];
        assert_eq!(have, want);
    }

    #[test]
    fn bash_file_with_arguments() {
        let command = bash("dir/test.bash --flag value").unwrap();
        let have = args(&command);
        let want = vec![
            "dir/test.bash".to_owned(),
            "--flag".to_owned(),
            "value".to_owned(),
        ];
        assert_eq!(have, want);
    }

    #[test]
    fn quoted_path_with_argument() {
        let command = bash(r#""my scripts/test.sh" "from bash""#).unwrap();
        let have = args(&command);
        let want = vec!["my scripts/test.sh".to_owned(), "from bash".to_owned()];
        assert_eq!(have, want);
    }

    #[test]
    fn windows_path() {
        let command = bash(r"C:\scripts\test.sh").unwrap();
        let have = args(&command);
        let want = vec![r"C:\scripts\test.sh".to_owned()];
        assert_eq!(have, want);
    }

    #[test]
    fn uppercase_extension() {
        let command = bash("DEMO.SH").unwrap();
        let have = args(&command);
        let want = vec!["DEMO.SH".to_owned()];
        assert_eq!(have, want);
    }

    #[test]
    fn extra_whitespace() {
        let command = bash("demo.sh    --flag").unwrap();
        let have = args(&command);
        let want = vec!["demo.sh".to_owned(), "--flag".to_owned()];
        assert_eq!(have, want);
    }

    #[test]
    fn quoted_metacharacter_is_an_argument() {
        let command = bash(r#"demo.sh "a&b""#).unwrap();
        let have = args(&command);
        let want = vec!["demo.sh".to_owned(), "a&b".to_owned()];
        assert_eq!(have, want);
    }

    #[test]
    fn other_command_uses_the_default_shell() {
        let mut looked_up = false;
        let have = bash_command("echo hello.sh", || {
            looked_up = true;
            Some(PathBuf::from("bash.exe"))
        });
        assert!(have.is_none());
        assert!(!looked_up);
    }

    #[test]
    fn shell_expression_uses_the_default_shell() {
        let mut looked_up = false;
        let have = bash_command("demo.sh && echo done", || {
            looked_up = true;
            Some(PathBuf::from("bash.exe"))
        });
        assert!(have.is_none());
        assert!(!looked_up);
    }

    #[test]
    fn other_extension() {
        let have = bash("notes.sh.txt");
        assert!(have.is_none());
    }

    #[test]
    fn without_git_bash() {
        let have = bash_command("demo.sh", || None);
        assert!(have.is_none());
    }

    #[test]
    fn empty() {
        let have = bash("   ");
        assert!(have.is_none());
    }

    #[test]
    fn unquoted_path_with_spaces() {
        let dir = tempfile::tempdir().unwrap();
        let script_dir = dir.path().join("my scripts");
        std::fs::create_dir_all(&script_dir).unwrap();
        let script = script_dir.join("hello.sh");
        std::fs::write(&script, "echo hi\n").unwrap();
        let runnable = script.to_str().unwrap();
        let command = bash(runnable).unwrap();
        let have = args(&command);
        let want = vec![runnable.to_owned()];
        assert_eq!(have, want);
    }
}
