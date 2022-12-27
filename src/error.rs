use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid Path \"{0}\"")]
    InvalidPath(String),
    #[error("Serde Json Error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Key not found: \"{0}\"")]
    KeyNotFound(String),
    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, KvError>;
