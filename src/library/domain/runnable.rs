use super::Executable;
use std::ops;

#[derive(Debug)]
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

    /// provides the names of all executables in this Runnable
    #[must_use]
    pub fn names(&self) -> Vec<&str> {
        match self {
            Runnable::Single(executable) => vec![&executable.name],
            Runnable::Sequence(executables) => executables
                .iter()
                .map(|executable| executable.name.as_str())
                .collect(),
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

    mod add {
        use crate::{Executable, Runnable};
        use std::process::Command;

        fn make_executable(name: &'static str) -> Executable {
            Executable {
                name: name.to_owned(),
                command: Command::new("true"),
            }
        }

        #[test]
        fn single_plus_single() {
            let single_a = Runnable::Single(make_executable("a"));
            let single_b = Runnable::Single(make_executable("b"));
            let have = single_a + single_b;
            assert_eq!(have.names(), ["a", "b"]);
        }

        #[test]
        fn sequence_plus_single() {
            let sequence = Runnable::Sequence(vec![make_executable("a"), make_executable("b")]);
            let single = Runnable::Single(make_executable("c"));
            let have = sequence + single;
            assert_eq!(have.names(), ["a", "b", "c"]);
        }

        #[test]
        fn single_plus_sequence() {
            let single = Runnable::Single(make_executable("a"));
            let sequence = Runnable::Sequence(vec![make_executable("b"), make_executable("c")]);
            let have = single + sequence;
            assert_eq!(have.names(), ["a", "b", "c"]);
        }

        #[test]
        fn sequence_plus_sequence() {
            let sequence_1 = Runnable::Sequence(vec![make_executable("a"), make_executable("b")]);
            let sequence_2 = Runnable::Sequence(vec![make_executable("c"), make_executable("d")]);
            let have = sequence_1 + sequence_2;
            assert_eq!(have.names(), ["a", "b", "c", "d"]);
        }

        #[test]
        fn empty_sequence_plus_single() {
            let sequence = Runnable::Sequence(vec![]);
            let single = Runnable::Single(make_executable("a"));
            let have = sequence + single;
            assert_eq!(have.len(), 1);
            assert_eq!(have.names(), ["a"]);
        }

        #[test]
        fn single_plus_empty_sequence() {
            let single = Runnable::Single(make_executable("a"));
            let sequence = Runnable::Sequence(vec![]);
            let have = single + sequence;
            assert_eq!(have.len(), 1);
            assert_eq!(have.names(), ["a"]);
        }

        #[test]
        fn empty_sequence_plus_empty_sequence() {
            let sequence_1 = Runnable::Sequence(vec![]);
            let sequence_2 = Runnable::Sequence(vec![]);
            let have = sequence_1 + sequence_2;
            assert_eq!(have.len(), 0);
            assert!(have.names().is_empty());
        }
    }
}
