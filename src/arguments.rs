use crate::commands::Command;
use crate::config::{Config, Show};
use crate::errors::UserError;

/// Parses command-line arguments into separate commands by splitting on the separator token.
pub fn parse<SI: Iterator<Item = String>>(args: SI) -> Result<Command, UserError> {
    let mut calls = vec![];
    let mut config = Config { show: Show::All };
    let mut parse_flags = true; // indicates whether we are still in the section that contains conc flags
    for arg in args {
        if arg == "--" {
            parse_flags = false;
            continue;
        }
        if !arg.starts_with('-') {
            parse_flags = false;
        }
        if parse_flags && arg.starts_with('-') {
            match arg.as_ref() {
                "--help" | "-h" => return Ok(Command::Help),
                "--show=all" | "--show" => config.show = Show::All,
                "--show=failed" => config.show = Show::Failed,
                "--version" | "-V" => return Ok(Command::Version),
                _ => return Err(UserError::UnknownFlag(arg)),
            }
            continue;
        }
        calls.push(arg.into());
    }
    Ok(Command::Run { config, calls })
}

#[cfg(test)]
mod tests {

    mod parse_commands {
        use super::super::*;
        use crate::subshell::Call;
        use big_s::S;

        #[test]
        fn single_command() {
            let give = vec![S("echo hello world")].into_iter();
            let have = parse(give).unwrap();
            let want = Command::Run {
                config: Config { show: Show::All },
                calls: vec![Call::from("echo hello world")],
            };
            assert_eq!(have, want);
        }

        #[test]
        fn multiple_commands() {
            let give = vec![S("echo hello"), S("ls -la"), S("pwd")].into_iter();
            let have = parse(give).unwrap();
            let want = Command::Run {
                config: Config { show: Show::All },
                calls: vec![
                    Call::from("echo hello"),
                    Call::from("ls -la"),
                    Call::from("pwd"),
                ],
            };
            assert_eq!(have, want);
        }

        #[test]
        fn empty() {
            let give = vec![].into_iter();
            let have = parse(give).unwrap();
            let want = Command::Run {
                config: Config { show: Show::All },
                calls: vec![],
            };
            assert_eq!(have, want);
        }

        #[test]
        fn show_failed() {
            let give = vec![S("--show=failed"), S("echo hello")].into_iter();
            let have = parse(give).unwrap();
            let want = Command::Run {
                config: Config { show: Show::Failed },
                calls: vec![Call::from("echo hello")],
            };
            assert_eq!(have, want);
        }

        #[test]
        fn show_all() {
            let give = vec![S("--show=all"), S("echo hello")].into_iter();
            let have = parse(give).unwrap();
            let want = Command::Run {
                config: Config { show: Show::All },
                calls: vec![Call::from("echo hello")],
            };
            assert_eq!(have, want);
        }

        #[test]
        fn unknown_flag() {
            let give = vec![S("--zonk"), S("echo hello")].into_iter();
            let have = parse(give);
            let want = Err(UserError::UnknownFlag(S("--zonk")));
            assert_eq!(have, want);
        }

        #[test]
        fn manually_end_flags_section() {
            let give = vec![S("--show"), S("--"), S("echo hello")].into_iter();
            let have = parse(give).unwrap();
            let want = Command::Run {
                config: Config { show: Show::All },
                calls: vec![Call::from("echo hello")],
            };
            assert_eq!(have, want);
        }

        #[test]
        fn help_long() {
            let give = vec![S("--help")].into_iter();
            let have = parse(give).unwrap();
            let want = Command::Help;
            assert_eq!(have, want);
        }

        #[test]
        fn help_short() {
            let give = vec![S("-h")].into_iter();
            let have = parse(give).unwrap();
            let want = Command::Help;
            assert_eq!(have, want);
        }

        #[test]
        fn version_short() {
            let give = vec![S("-V")].into_iter();
            let have = parse(give).unwrap();
            let want = Command::Version;
            assert_eq!(have, want);
        }

        #[test]
        fn version() {
            let give = vec![S("--version")].into_iter();
            let have = parse(give).unwrap();
            let want = Command::Version;
            assert_eq!(have, want);
        }
    }
}
