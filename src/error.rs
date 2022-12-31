use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Invalid Path \"{0}\"")]
    InvalidPath(String),
    #[error("Key not found: \"{0}\"")]
    KeyNotFound(String),
    #[error("Bincode Error: {0}")]
    BincodeError(#[from] bincode::Error),
    #[error("End Of File")]
    EOF,
    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, KvError>;
