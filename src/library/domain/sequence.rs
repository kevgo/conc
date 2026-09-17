use super::Executable;
use std::ops;

/// one or multiple commands executing in sequence
#[derive(Debug, PartialEq)]
pub struct Sequence {
    first: Executable,

    additional: Vec<Executable>,
}

impl Sequence {
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

impl From<Executable> for Sequence {
    fn from(executable: Executable) -> Sequence {
        Sequence {
            first: executable,
            additional: vec![],
        }
    }
}

impl TryFrom<Vec<Executable>> for Sequence {
    type Error = String;

    fn try_from(mut value: Vec<Executable>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("cannot create a sequence from an empty vector".to_owned())
        } else {
            Ok(Sequence {
                first: value.remove(0),
                additional: value,
            })
        }
    }
}

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
            let single_a = Sequence::from(make_executable("a"));
            let single_b = Sequence::from(make_executable("b"));
            let have = single_a + single_b;
            let want =
                Sequence::try_from(vec![make_executable("a"), make_executable("b")]).unwrap();
            assert_eq!(have, want);
        }

        #[test]
        fn sequence_plus_single() {
            let sequence =
                Sequence::try_from(vec![make_executable("a"), make_executable("b")]).unwrap();
            let single = Sequence::from(make_executable("c"));
            let have = sequence + single;
            let want = Sequence::try_from(vec![
                make_executable("a"),
                make_executable("b"),
                make_executable("c"),
            ])
            .unwrap();
            assert_eq!(have, want);
        }

        #[test]
        fn single_plus_sequence() {
            let single = Sequence::from(make_executable("a"));
            let sequence =
                Sequence::try_from(vec![make_executable("b"), make_executable("c")]).unwrap();
            let have = single + sequence;
            let want = Sequence::try_from(vec![
                make_executable("a"),
                make_executable("b"),
                make_executable("c"),
            ])
            .unwrap();
            assert_eq!(have, want);
        }

        #[test]
        fn sequence_plus_sequence() {
            let sequence_1 =
                Sequence::try_from(vec![make_executable("a"), make_executable("b")]).unwrap();
            let sequence_2 =
                Sequence::try_from(vec![make_executable("c"), make_executable("d")]).unwrap();
            let have = sequence_1 + sequence_2;
            let want = Sequence::try_from(vec![
                make_executable("a"),
                make_executable("b"),
                make_executable("c"),
                make_executable("d"),
            ])
            .unwrap();
            assert_eq!(have, want);
        }
    }

    mod len {
        use super::make_executable;
        use crate::Sequence;

        #[test]
        fn one() {
            assert_eq!(Sequence::from(make_executable("")).len(), 1);
        }

        #[test]
        fn many() {
            assert_eq!(
                Sequence::try_from(vec![
                    make_executable(""),
                    make_executable(""),
                    make_executable(""),
                ])
                .unwrap()
                .len(),
                3
            );
        }
    }

    mod into_iter {
        use super::make_executable;
        use crate::Sequence;

        #[test]
        fn one() {
            let sequence = Sequence::from(make_executable("a"));
            let have: Vec<_> = sequence.into_iter().collect();
            let want = vec![make_executable("a")];
            assert_eq!(have, want);
        }

        #[test]
        fn many() {
            let sequence = Sequence::try_from(vec![
                make_executable("a"),
                make_executable("b"),
                make_executable("c"),
            ])
            .unwrap();
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
