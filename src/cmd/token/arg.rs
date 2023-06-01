use std::fmt;

#[derive(Clone, Debug)]
pub enum Arg {
    Values,
    Entries
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arg::Values => write!(f, "values"),
            Arg::Entries => write!(f, "entries"),
        }
    }
}