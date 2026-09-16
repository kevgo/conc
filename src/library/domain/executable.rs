use std::fmt::Debug;
use std::process::Command;

/// all the information Conc needs to execute a single command
pub struct Executable {
    /// how the command will be displayed
    pub name: String,

    /// the command to execute
    pub command: Command,
}

impl Executable {
    /// the program and arguments that will be executed
    pub(crate) fn command_line(&self) -> String {
        let mut result = self.command.get_program().to_string_lossy().into_owned();
        for arg in self.command.get_args() {
            result.push(' ');
            let arg_str = arg.to_string_lossy();
            match shlex::try_quote(&arg_str) {
                Ok(quoted) => result.push_str(&quoted),
                Err(_) => result.push_str(&arg_str),
            }
        }
        result
    }
}

impl Debug for Executable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\
Executable {{
    name: {name}
    command: {command:?}
}}",
            name = self.name,
            command = self.command,
        )
    }
}

impl PartialEq for Executable {
    fn eq(&self, other: &Executable) -> bool {
        let Executable {
            name: self_name,
            command: self_command,
        } = self;
        let Executable {
            name: other_name,
            command: other_command,
        } = other;
        let name_match = self_name == other_name;
        let program_match = self_command.get_program() == other_command.get_program();
        let args_match = self_command.get_args().eq(other_command.get_args());
        let cwd_match = self_command.get_current_dir() == other_command.get_current_dir();
        let env_match = self_command.get_envs().eq(other_command.get_envs());
        name_match && program_match && args_match && cwd_match && env_match
    }
}

#[cfg(test)]
mod tests {
    mod command_line {
        use crate::{Executable, shell_executable};
        use big_s::S;
        use std::process::Command;

        #[test]
        fn raw_command() {
            let mut command = Command::new("echo");
            command.args(vec!["single", "two words"]);
            let executable = Executable {
                name: S("test"),
                command,
            };
            let have = executable.command_line();
            let want = "echo single 'two words'";
            assert_eq!(have, want);
        }

        #[test]
        #[cfg(unix)]
        fn shell_command() {
            let executable = shell_executable("echo single \"two words\"");
            let have = executable.command_line();
            let want = "sh -c 'echo single \"two words\"'";
            assert_eq!(have, want);
        }

        #[test]
        #[cfg(windows)]
        fn shell_command() {
            let executable = shell_executable("echo single \"two words\"");
            let have = executable.command_line();
            let want = "cmd.exe /C 'echo single \"two words\"'";
            assert_eq!(have, want);
        }
    }

    mod partial_eq {
        use crate::Executable;
        use big_s::S;
        use std::process::Command;

        fn make_executable() -> Executable {
            let mut command = Command::new("echo");
            command.arg("hello");
            Executable {
                name: S("test"),
                command,
            }
        }

        #[test]
        fn equal() {
            let have = make_executable();
            let want = make_executable();
            assert_eq!(have, want);
        }

        #[test]
        fn equal_with_cwd_and_env() {
            let mut have = make_executable();
            have.command.current_dir("dir");
            have.command.env("FOO", "bar");
            let mut want = make_executable();
            want.command.current_dir("dir");
            want.command.env("FOO", "bar");
            assert_eq!(have, want);
        }

        #[test]
        fn different_name() {
            let have = make_executable();
            let mut want = make_executable();
            want.name = S("other");
            assert_ne!(have, want);
        }

        #[test]
        fn different_program() {
            let have = make_executable();
            let mut command = Command::new("cat");
            command.arg("hello");
            let want = Executable {
                name: S("test"),
                command,
            };
            assert_ne!(have, want);
        }

        #[test]
        fn different_args() {
            let have = make_executable();
            let mut want = make_executable();
            want.command.arg("world");
            assert_ne!(have, want);
        }

        #[test]
        fn different_cwd() {
            let mut have = make_executable();
            have.command.current_dir("dir_a");
            let mut want = make_executable();
            want.command.current_dir("dir_b");
            assert_ne!(have, want);
        }

        #[test]
        fn different_env() {
            let mut have = make_executable();
            have.command.env("FOO", "bar");
            let mut want = make_executable();
            want.command.env("FOO", "baz");
            assert_ne!(have, want);
        }
    }

    mod debug {
        use crate::{Executable, shell_executable};
        use big_s::S;
        use std::process::Command;

        #[test]
        fn raw_command() {
            let mut command = Command::new("echo");
            command.args(vec!["single", "two words"]);
            let executable = Executable {
                name: S("test"),
                command,
            };
            let have = format!("{executable:?}");
            let want = format!(
                "\
Executable {{
    name: test
    command: {:?}
}}",
                executable.command
            );
            assert_eq!(have, want);
        }

        #[test]
        fn shell_command() {
            let executable = shell_executable("echo single \"two words\"");
            let have = format!("{executable:?}");
            let want = format!(
                "\
Executable {{
    name: echo single \"two words\"
    command: {:?}
}}",
                executable.command
            );
            assert_eq!(have, want);
        }
    }
}
