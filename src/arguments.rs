use crate::config::{Config, Show};
use crate::errors::UserError;
use crate::subshell::Call;

/// separates different commands in the CLI arguments
const SEPARATOR: &str = "}{";

/// Parses command-line arguments into separate commands by splitting on the separator token.
pub(crate) fn parse_commands(
    args: impl Iterator<Item = String>,
) -> Result<(Config, Vec<Call>), UserError> {
    let mut result = vec![];
    let mut executable = None;
    let mut arguments = vec![];
    let mut show = Show::All;
    let mut parse_flags = true; // indicates whether we are still in the section that contains conc flags
    for arg in args {
        if arg == "--" {
            parse_flags = false;
            continue;
        }
        if !arg.starts_with("--") {
            parse_flags = false;
        }
        if parse_flags && arg.starts_with("-") {
            match arg.as_ref() {
                "--show=all" | "--show" => {
                    show = Show::All;
                    continue;
                }
                "--show=failed" => {
                    show = Show::Failed;
                    continue;
                }
                _ => return Err(UserError::UnknownFlag(arg)),
            }
        };
        if arg == SEPARATOR {
            if let Some(executable) = executable {
                result.push(Call {
                    executable,
                    arguments,
                });
            }
            executable = None;
            arguments = vec![];
        } else if executable.is_none() {
            executable = Some(arg);
        } else {
            arguments.push(arg);
        }
    }
    if let Some(executable) = executable {
        result.push(Call {
            executable,
            arguments,
        });
    }
    Ok((Config { show }, result))
}

#[cfg(test)]
mod tests {

    mod parse_commands {
        use super::super::*;
        use big_s::S;

        #[test]
        fn single_command() {
            let give = vec![S("echo"), S("hello"), S("world")].into_iter();
            let have = parse_commands(give).unwrap();
            let want = (
                Config { show: Show::All },
                vec![Call {
                    executable: S("echo"),
                    arguments: vec![S("hello"), S("world")],
                }],
            );
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
            let have = parse_commands(give).unwrap();
            let want = (
                Config { show: Show::All },
                vec![
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
                ],
            );
            assert_eq!(have, want);
        }

        #[test]
        fn empty() {
            let give = vec![].into_iter();
            let have = parse_commands(give).unwrap();
            let want = (Config { show: Show::All }, vec![]);
            assert_eq!(have, want);
        }

        #[test]
        fn outside_separators() {
            let give = vec![S("}{"), S("echo"), S("hello"), S("}{")].into_iter();
            let have = parse_commands(give).unwrap();
            let want = (
                Config { show: Show::All },
                vec![Call {
                    executable: S("echo"),
                    arguments: vec![S("hello")],
                }],
            );
            assert_eq!(have, want);
        }

        #[test]
        fn consecutive_separators() {
            let give = vec![S("echo"), S("hello"), S("}{"), S("}{"), S("}{"), S("pwd")].into_iter();
            let have = parse_commands(give).unwrap();
            let want = (
                Config { show: Show::All },
                vec![
                    Call {
                        executable: S("echo"),
                        arguments: vec![S("hello")],
                    },
                    Call {
                        executable: S("pwd"),
                        arguments: vec![],
                    },
                ],
            );
            assert_eq!(have, want);
        }

        #[test]
        fn show_failed() {
            let give = vec![S("--show=failed"), S("echo"), S("hello")].into_iter();
            let have = parse_commands(give).unwrap();
            let want = (
                Config { show: Show::Failed },
                vec![Call {
                    executable: S("echo"),
                    arguments: vec![S("hello")],
                }],
            );
            assert_eq!(have, want);
        }

        #[test]
        fn show_all() {
            let give = vec![S("--show=all"), S("echo"), S("hello")].into_iter();
            let have = parse_commands(give).unwrap();
            let want = (
                Config { show: Show::All },
                vec![Call {
                    executable: S("echo"),
                    arguments: vec![S("hello")],
                }],
            );
            assert_eq!(have, want);
        }

        #[test]
        fn unknown_flag() {
            let give = vec![S("--zonk"), S("echo"), S("hello")].into_iter();
            let have = parse_commands(give);
            let want = Err(UserError::UnknownFlag(S("--zonk")));
            assert_eq!(have, want);
        }

        #[test]
        fn manually_end_flags_section() {
            let give = vec![S("--show"), S("--"), S("echo"), S("hello")].into_iter();
            let have = parse_commands(give).unwrap();
            let want = (
                Config { show: Show::All },
                vec![Call {
                    executable: S("echo"),
                    arguments: vec![S("hello")],
                }],
            );
            assert_eq!(have, want);
        }
    }
}
