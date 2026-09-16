/// all information Conc needs to execute a command
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
