use thiserror::Error;
use std::{
    string::FromUtf8Error,
    array::TryFromSliceError,
};

#[derive(Error, Debug)]
pub enum KvError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Path '{0}' is a folder")]
    IsDir(String),
    #[error("Invalid Path '{0}'")]
    InvalidPath(String),
    #[error("Key not found: \"{0}\"")]
    KeyNotFound(String),
    #[error("{0}")]
    BincodeError(#[from] Box<bincode::ErrorKind>),
    #[error("FromUtf8 Error: {0}")]
    DecodeUtf8Error(#[from] FromUtf8Error),
    #[error("Slice Decode Error: {0}")]
    SliceDecodeError(#[from] TryFromSliceError),
    #[error("{0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("{0}")]
    TomlSeError(#[from] toml::ser::Error),
    #[error("End Of File")]
    EOF,
    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, KvError>;
