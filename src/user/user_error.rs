use thiserror::Error;
use std::str::Utf8Error;

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
    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("{0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("{0}")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("{0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("{0}")]
    DecodeUtf8Error(#[from] Utf8Error),
    #[error("The timestamp is abnormal, maybe the clock is back")]
    ClockBack,
    #[error("Unqualified length of data machine id")]
    WorkerIdLengthError,
    #[error("Unqualified length of data center id")]
    DataCenterLengthError,
    #[error("User '{0}' not found")]
    UserNotFound(String),
    #[error("username '{0}' exists")]
    UserNameExists(String),
}

pub type Result<T> = std::result::Result<T, UserError>;
