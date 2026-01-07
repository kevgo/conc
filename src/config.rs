#[derive(Debug, Default, PartialEq, Clone)]
pub(crate) struct Config {
    pub show: Show,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Show {
    All,
    Failed,
}

impl Default for Show {
    fn default() -> Self {
        Show::All
    }
}
