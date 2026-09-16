use super::Executable;
use std::ops;

/// one or multiple commands executing in sequence
// TODO: rename to Sequence and make it have one guaranteed command plus a vec of additional commands
#[derive(Debug, PartialEq)]
pub enum Runnable {
    /// run a single command
    Single(Executable),

    /// run the given commands one after the other
    Sequence(Vec<Executable>),
}

impl Runnable {
    /// indicates whether the runnable executes any commands
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// returns the number of commands in the runnable
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Runnable::Single(_) => 1,
            Runnable::Sequence(executables) => executables.len(),
        }
    }
}

impl ops::Add for Runnable {
    type Output = Runnable;

    fn add(self, other: Runnable) -> Runnable {
        match (self, other) {
            (Runnable::Single(mine), Runnable::Single(other)) => {
                Runnable::Sequence(vec![mine, other])
            }
            (Runnable::Sequence(mine), Runnable::Single(other)) => {
                let mut result = Vec::with_capacity(mine.len() + 1);
                result.extend(mine);
                result.push(other);
                Runnable::Sequence(result)
            }
            (Runnable::Single(mine), Runnable::Sequence(other)) => {
                let mut result = Vec::with_capacity(other.len() + 1);
                result.push(mine);
                result.extend(other);
                Runnable::Sequence(result)
            }
            (Runnable::Sequence(mine), Runnable::Sequence(other)) => {
                let mut result = Vec::with_capacity(mine.len() + other.len());
                result.extend(mine);
                result.extend(other);
                Runnable::Sequence(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Executable;
    use std::process::Command;

    fn make_executable(name: &'static str) -> Executable {
        Executable {
            name: name.to_owned(),
            command: Command::new("true"),
        }
    }

    mod add {
        use super::make_executable;
        use crate::Runnable;

        #[test]
        fn single_plus_single() {
            let single_a = Runnable::Single(make_executable("a"));
            let single_b = Runnable::Single(make_executable("b"));
            let have = single_a + single_b;
            let want = Runnable::Sequence(vec![make_executable("a"), make_executable("b")]);
            assert_eq!(have, want);
        }

        #[test]
        fn sequence_plus_single() {
            let sequence = Runnable::Sequence(vec![make_executable("a"), make_executable("b")]);
            let single = Runnable::Single(make_executable("c"));
            let have = sequence + single;
            let want = Runnable::Sequence(vec![
                make_executable("a"),
                make_executable("b"),
                make_executable("c"),
            ]);
            assert_eq!(have, want);
        }

        #[test]
        fn single_plus_sequence() {
            let single = Runnable::Single(make_executable("a"));
            let sequence = Runnable::Sequence(vec![make_executable("b"), make_executable("c")]);
            let have = single + sequence;
            let want = Runnable::Sequence(vec![
                make_executable("a"),
                make_executable("b"),
                make_executable("c"),
            ]);
            assert_eq!(have, want);
        }

        #[test]
        fn sequence_plus_sequence() {
            let sequence_1 = Runnable::Sequence(vec![make_executable("a"), make_executable("b")]);
            let sequence_2 = Runnable::Sequence(vec![make_executable("c"), make_executable("d")]);
            let have = sequence_1 + sequence_2;
            let want = Runnable::Sequence(vec![
                make_executable("a"),
                make_executable("b"),
                make_executable("c"),
                make_executable("d"),
            ]);
            assert_eq!(have, want);
        }

        #[test]
        fn empty_sequence_plus_single() {
            let sequence = Runnable::Sequence(vec![]);
            let single = Runnable::Single(make_executable("a"));
            let have = sequence + single;
            assert_eq!(have.len(), 1);
            let want = Runnable::Sequence(vec![make_executable("a")]);
            assert_eq!(have, want);
        }

        #[test]
        fn single_plus_empty_sequence() {
            let single = Runnable::Single(make_executable("a"));
            let sequence = Runnable::Sequence(vec![]);
            let have = single + sequence;
            assert_eq!(have.len(), 1);
            let want = Runnable::Sequence(vec![make_executable("a")]);
            assert_eq!(have, want);
        }

        #[test]
        fn empty_sequence_plus_empty_sequence() {
            let sequence_1 = Runnable::Sequence(vec![]);
            let sequence_2 = Runnable::Sequence(vec![]);
            let have = sequence_1 + sequence_2;
            assert_eq!(have.len(), 0);
            let want = Runnable::Sequence(vec![]);
            assert_eq!(have, want);
        }
    }

    mod is_empty {
        use super::make_executable;
        use crate::Runnable;

        #[test]
        fn single_is_never_empty() {
            assert!(!Runnable::Single(make_executable("")).is_empty());
        }

        #[test]
        fn empty_sequence_is_empty() {
            assert!(Runnable::Sequence(vec![]).is_empty());
        }

        #[test]
        fn non_empty_sequence_is_not_empty() {
            assert!(!Runnable::Sequence(vec![make_executable("")]).is_empty());
        }
    }

    mod len {
        use super::make_executable;
        use crate::Runnable;

        #[test]
        fn single_is_always_one() {
            assert_eq!(Runnable::Single(make_executable("")).len(), 1);
        }

        #[test]
        fn empty_sequence_is_zero() {
            assert_eq!(Runnable::Sequence(vec![]).len(), 0);
        }

        #[test]
        fn sequence_of_one() {
            assert_eq!(Runnable::Sequence(vec![make_executable("")]).len(), 1);
        }

        #[test]
        fn sequence_of_many() {
            assert_eq!(
                Runnable::Sequence(vec![
                    make_executable(""),
                    make_executable(""),
                    make_executable(""),
                ])
                .len(),
                3
            );
        }
    }
}
