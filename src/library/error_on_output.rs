/// Whether to error if any command produces output.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ErrorOnOutput(bool);

impl From<bool> for ErrorOnOutput {
    fn from(value: bool) -> Self {
        ErrorOnOutput(value)
    }
}

impl ErrorOnOutput {
    pub(crate) fn enabled(self) -> bool {
        self.0
    }
}
