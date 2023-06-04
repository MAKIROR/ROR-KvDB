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
    #[error("{0}")]
    SliceDecodeError(#[from] TryFromSliceError),
    #[error("{0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("{0}")]
    TomlSeError(#[from] toml::ser::Error),
    #[error("Cannot convert '{0}' to {1}")]
    ConvertError(String,String),
    #[error("Incorrect argument to command '{0}'")]
    ParameterError(String),
    #[error("Unknown type '{0}'")]
    UnknownType(String),
    #[error("Unknown command '{0}'")]
    UnknownCommand(String),
    #[error("End Of File")]
    EOF,
    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, KvError>;
