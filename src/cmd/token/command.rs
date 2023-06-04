use std::fmt;

#[derive(Clone, Debug)]
pub enum Command {
    Open,
    Add,
    Delete,
    Compact,
    Get,
    TypeOf,
    List,
    User,
    Quit,
    Create
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Open => write!(f, "open"),
            Command::Add => write!(f, "add"),
            Command::Delete => write!(f, "delete"),
            Command::Compact => write!(f, "compact"),
            Command::Get => write!(f, "get"),
            Command::TypeOf => write!(f, "typeof"),
            Command::List => write!(f, "list"),
            Command::User => write!(f, "user"),
            Command::Quit => write!(f, "quit"),
            Command::Create => write!(f, "create"),
        }
    }
}
