use std::fmt::Display;

/// separates different commands in the CLI arguments
const SEPARATOR: &str = "}{";

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Call {
    pub(crate) executable: String,
    pub(crate) arguments: Vec<String>,
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.executable, self.arguments.join(" "))
    }
}

/// Parses command-line arguments into separate commands by splitting on the separator token.
pub(crate) fn parse_commands(args: impl Iterator<Item = String>) -> Vec<Call> {
    let mut result = vec![];
    let mut current_executable = None;
    let mut current_arguments = vec![];
    for arg in args {
        if arg == SEPARATOR {
            if let Some(executable) = current_executable {
                result.push(Call {
                    executable,
                    arguments: current_arguments,
                });
            }
            current_executable = None;
            current_arguments = vec![];
        } else {
            if current_executable.is_none() {
                current_executable = Some(arg);
            } else {
                current_arguments.push(arg);
            }
        }
    }
    // Push the final command if there is one
    if let Some(executable) = current_executable {
        result.push(Call {
            executable,
            arguments: current_arguments,
        });
    }
    result
}

#[cfg(test)]
mod tests {

    mod parse_commands {
        use super::super::*;
        use big_s::S;

        #[test]
        fn single_command() {
            let give = vec![S("echo"), S("hello")].into_iter();
            let have = parse_commands(give);
            let want = vec![Call {
                executable: S("echo"),
                arguments: vec![S("hello")],
            }];
            assert_eq!(have, want);
        }

        #[test]
        fn multiple_commands() {
            let give = vec![
                S("echo"),
                S("hello"),
                S("}{"),
                S("ls"),
                S("-la"),
                S("}{"),
                S("pwd"),
            ]
            .into_iter();
            let have = parse_commands(give);
            let want = vec![
                Call {
                    executable: S("echo"),
                    arguments: vec![S("hello")],
                },
                Call {
                    executable: S("ls"),
                    arguments: vec![S("-la")],
                },
                Call {
                    executable: S("pwd"),
                    arguments: vec![],
                },
            ];
            assert_eq!(have, want);
        }

        #[test]
        fn empty() {
            let give = vec![].into_iter();
            let have = parse_commands(give);
            let want = vec![];
            assert_eq!(have, want);
        }

        #[test]
        fn trailing_separator() {
            let give = vec![S("echo"), S("hello"), S("}{")].into_iter();
            let have = parse_commands(give);
            let want = vec![Call {
                executable: S("echo"),
                arguments: vec![S("hello")],
            }];
            assert_eq!(have, want);
        }

        #[test]
        fn leading_separator() {
            let give = vec![S("}{"), S("echo"), S("hello")].into_iter();
            let have = parse_commands(give);
            let want = vec![Call {
                executable: S("echo"),
                arguments: vec![S("hello")],
            }];
            assert_eq!(have, want);
        }

        #[test]
        fn consecutive_separators() {
            let give = vec![S("echo"), S("hello"), S("}{"), S("}{"), S("pwd")].into_iter();
            let have = parse_commands(give);
            let want = vec![
                Call {
                    executable: S("echo"),
                    arguments: vec![S("hello")],
                },
                Call {
                    executable: S("pwd"),
                    arguments: vec![],
                },
            ];
            assert_eq!(have, want);
        }
    }
}
