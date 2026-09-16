use std::fmt::Debug;
use std::process::Command;

/// all the information Conc needs to execute a command
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
            let arg_str_2 = arg_str.clone();
            let quoted = shlex::try_quote(&arg_str).unwrap_or(arg_str_2);
            result.push_str(&quoted);
        }
        result
    }
}

impl Debug for Executable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"Executable {{
    name: {}
    command: {:?}
}}",
            self.name, self.command
        )
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
            command.args(vec!["one", "two three"]);
            let executable = Executable {
                name: S("test"),
                command,
            };
            let have = executable.command_line();
            let want = "echo one 'two three'";
            assert_eq!(have, want);
        }

        #[test]
        #[cfg(unix)]
        fn shell_command() {
            let executable = shell_executable("echo one \"two three\"");
            let have = executable.command_line();
            let want = "sh -c 'echo one \"two three\"'";
            assert_eq!(have, want);
        }
    }
}
