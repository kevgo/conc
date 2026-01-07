use crate::subshell::Call;

/// separates different commands in the CLI arguments
const SEPARATOR: &str = "}{";

/// Parses command-line arguments into separate commands by splitting on the separator token.
pub(crate) fn parse_commands(args: impl Iterator<Item = String>) -> Vec<Call> {
    let mut result = vec![];
    let mut executable = None;
    let mut arguments = vec![];
    for arg in args {
        if arg == SEPARATOR {
            if let Some(executable) = executable {
                result.push(Call {
                    executable,
                    arguments,
                });
            }
            executable = None;
            arguments = vec![];
        } else {
            if executable.is_none() {
                executable = Some(arg);
            } else {
                arguments.push(arg);
            }
        }
    }
    if let Some(executable) = executable {
        result.push(Call {
            executable,
            arguments,
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
            let give = vec![S("echo"), S("hello"), S("world")].into_iter();
            let have = parse_commands(give);
            let want = vec![Call {
                executable: S("echo"),
                arguments: vec![S("hello"), S("world")],
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
        fn outside_separators() {
            let give = vec![S("}{"), S("echo"), S("hello"), S("}{")].into_iter();
            let have = parse_commands(give);
            let want = vec![Call {
                executable: S("echo"),
                arguments: vec![S("hello")],
            }];
            assert_eq!(have, want);
        }

        #[test]
        fn consecutive_separators() {
            let give = vec![S("echo"), S("hello"), S("}{"), S("}{"), S("}{"), S("pwd")].into_iter();
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
