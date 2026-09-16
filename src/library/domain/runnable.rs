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
