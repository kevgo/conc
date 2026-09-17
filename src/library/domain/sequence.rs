use super::Executable;
use std::ops;

/// one or multiple commands executing in sequence
#[allow(clippy::len_without_is_empty)]
#[derive(Debug, PartialEq)]
pub struct Sequence {
    first: Executable,

    additional: Vec<Executable>,
}

impl Sequence {
    #[must_use]
    pub fn one(executable: Executable) -> Sequence {
        Sequence {
            first: executable,
            additional: vec![],
        }
    }

    #[must_use]
    pub fn many(first: Executable, additional: Vec<Executable>) -> Sequence {
        Sequence { first, additional }
    }

    /// returns the number of commands in the runnable
    #[must_use]
    pub fn len(&self) -> usize {
        self.additional.len() + 1
    }
}

impl IntoIterator for Sequence {
    type Item = Executable;
    type IntoIter = SequenceIter;

    fn into_iter(self) -> Self::IntoIter {
        SequenceIter {
            first: std::iter::once(self.first),
            additional: self.additional.into_iter(),
        }
    }
}

/// owning iterator over the commands in a [`Sequence`]
pub struct SequenceIter {
    first: std::iter::Once<Executable>,
    additional: std::vec::IntoIter<Executable>,
}

impl Iterator for SequenceIter {
    type Item = Executable;

    fn next(&mut self) -> Option<Self::Item> {
        self.first.next().or_else(|| self.additional.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.first.len() + self.additional.len();
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for SequenceIter {}

impl ops::Add for Sequence {
    type Output = Sequence;

    fn add(self, other: Sequence) -> Sequence {
        let mut new_additional = Vec::with_capacity(self.additional.len() + other.len());
        let Sequence {
            first: self_first,
            additional: self_additional,
        } = self;
        let Sequence {
            first: other_first,
            additional: other_additional,
        } = other;
        new_additional.extend(self_additional);
        new_additional.push(other_first);
        new_additional.extend(other_additional);
        Sequence {
            first: self_first,
            additional: new_additional,
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
        use crate::Sequence;

        #[test]
        fn single_plus_single() {
            let single_a = Sequence::one(make_executable("a"));
            let single_b = Sequence::one(make_executable("b"));
            let have = single_a + single_b;
            let want = Sequence {
                first: make_executable("a"),
                additional: vec![make_executable("b")],
            };
            assert_eq!(have, want);
        }

        #[test]
        fn sequence_plus_single() {
            let sequence = Sequence {
                first: make_executable("a"),
                additional: vec![make_executable("b")],
            };
            let single = Sequence::one(make_executable("c"));
            let have = sequence + single;
            let want = Sequence::many(
                make_executable("a"),
                vec![make_executable("b"), make_executable("c")],
            );
            assert_eq!(have, want);
        }

        #[test]
        fn single_plus_sequence() {
            let single = Sequence::one(make_executable("a"));
            let sequence = Sequence::many(make_executable("b"), vec![make_executable("c")]);
            let have = single + sequence;
            let want = Sequence::many(
                make_executable("a"),
                vec![make_executable("b"), make_executable("c")],
            );
            assert_eq!(have, want);
        }

        #[test]
        fn sequence_plus_sequence() {
            let sequence_1 = Sequence::many(make_executable("a"), vec![make_executable("b")]);
            let sequence_2 = Sequence::many(make_executable("c"), vec![make_executable("d")]);
            let have = sequence_1 + sequence_2;
            let want = Sequence::many(
                make_executable("a"),
                vec![
                    make_executable("b"),
                    make_executable("c"),
                    make_executable("d"),
                ],
            );
            assert_eq!(have, want);
        }
    }

    mod len {
        use super::make_executable;
        use crate::Sequence;

        #[test]
        fn one() {
            let give = Sequence::one(make_executable(""));
            assert_eq!(give.len(), 1);
        }

        #[test]
        fn many() {
            let give = Sequence::many(
                make_executable(""),
                vec![make_executable(""), make_executable("")],
            );
            assert_eq!(give.len(), 3);
        }
    }

    mod into_iter {
        use super::make_executable;
        use crate::Sequence;

        #[test]
        fn one() {
            let sequence = Sequence::one(make_executable("a"));
            let have: Vec<_> = sequence.into_iter().collect();
            let want = vec![make_executable("a")];
            assert_eq!(have, want);
        }

        #[test]
        fn many() {
            let sequence = Sequence::many(
                make_executable("a"),
                vec![make_executable("b"), make_executable("c")],
            );
            let have: Vec<_> = sequence.into_iter().collect();
            let want = vec![
                make_executable("a"),
                make_executable("b"),
                make_executable("c"),
            ];
            assert_eq!(have, want);
        }
    }
}
