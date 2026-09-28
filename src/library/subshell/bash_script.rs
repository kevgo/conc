//! Decides when a Windows command should run through Git Bash.
//!
//! A command runs through Git Bash when it invokes a `.sh` or `.bash` file,
//! optionally followed by arguments.
//! Shell expressions such as `script.sh && echo done` keep using `cmd.exe`.

use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) fn is_bash_expression(runnable: &str) -> bool {
    let Some(words) = shlex::split(runnable) else {
        return false;
    };

    // the first word ends with .sh or .bash --> is a bash expression
    let Some(first_word) = words.first() else {
        return false;
    };
    if has_bash_extension(first_word) {
        return true;
    }

    // the expression contains bashisms --> is a bash expression
    words.iter().any(|word| is_bashism(word))
}

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

fn is_bashism(word: &str) -> bool {
    matches!(word, "&&" | "||" | ";" | ">" | ">>" | "<" | "<<")
}

fn has_bash_extension(program: &str) -> bool {
    Path::new(program)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("sh") || extension.eq_ignore_ascii_case("bash")
        })
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
        let command = bash(r#""C:\scripts\test.sh""#).unwrap();
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
}
