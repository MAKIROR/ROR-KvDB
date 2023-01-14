use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Unknown user level '{0}'")]
    UnknownLevel(String),
    #[error("incorrect password format '{0}'")]
    PassWordFormatError(String),
    #[error("User name length is {0}, the length of the name should be between 2-20")]
    NameLengthError(usize),
    #[error("{0}")]
    RegexError(#[from] regex::Error),
    #[error("{0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("The timestamp is abnormal, maybe the clock is back")]
    ClockBack,
    #[error("The length of machine id should be between 0-31")]
    MachineIdLengthError,
    #[error("{0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
}

pub type Result<T> = std::result::Result<T, UserError>;
