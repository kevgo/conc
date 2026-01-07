use crate::config::{Config, Show};
use crate::errors::UserError;
use crate::subshell::Call;

/// Parses command-line arguments into separate commands by splitting on the separator token.
pub(crate) fn parse(args: impl Iterator<Item = String>) -> Result<(Config, Vec<Call>), UserError> {
    let mut calls = vec![];
    let mut show = Show::All;
    let mut parse_flags = true; // indicates whether we are still in the section that contains conc flags
    for arg in args {
        if arg == "--" {
            parse_flags = false;
            continue;
        }
        if !arg.starts_with("-") {
            parse_flags = false;
        }
        if parse_flags && arg.starts_with("-") {
            if arg == "--show=all" || arg == "--show" {
                show = Show::All;
                continue;
            } else if arg == "--show=failed" {
                show = Show::Failed;
                continue;
            }
            return Err(UserError::UnknownFlag(arg));
        }
        calls.push(arg.into());
    }
    Ok((Config { show }, calls))
}

#[cfg(test)]
mod tests {

    mod parse_commands {
        use super::super::*;
        use big_s::S;

        #[test]
        fn single_command() {
            let give = vec![S("echo hello world")].into_iter();
            let have = parse(give).unwrap();
            let want = (
                Config { show: Show::All },
                vec![Call::from("echo hello world")],
            );
            assert_eq!(have, want);
        }

        #[test]
        fn multiple_commands() {
            let give = vec![S("echo hello"), S("ls -la"), S("pwd")].into_iter();
            let have = parse(give).unwrap();
            let want = (
                Config { show: Show::All },
                vec![
                    Call::from("echo hello"),
                    Call::from("ls -la"),
                    Call::from("pwd"),
                ],
            );
            assert_eq!(have, want);
        }

        #[test]
        fn empty() {
            let give = vec![].into_iter();
            let have = parse(give).unwrap();
            let want = (Config { show: Show::All }, vec![]);
            assert_eq!(have, want);
        }

        #[test]
        fn show_failed() {
            let give = vec![S("--show=failed"), S("echo hello")].into_iter();
            let have = parse(give).unwrap();
            let want = (
                Config { show: Show::Failed },
                vec![Call::from("echo hello")],
            );
            assert_eq!(have, want);
        }

        #[test]
        fn show_all() {
            let give = vec![S("--show=all"), S("echo hello")].into_iter();
            let have = parse(give).unwrap();
            let want = (Config { show: Show::All }, vec![Call::from("echo hello")]);
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
            let want = (Config { show: Show::All }, vec![Call::from("echo hello")]);
            assert_eq!(have, want);
        }
    }
}
