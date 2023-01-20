use thiserror::Error;
use std::sync::{MutexGuard,PoisonError};
use super::{
    store::{
        kv_error::KvError,
        kv::DataStore,
    },
    user::user_error::UserError,
};

#[derive(Error, Debug)]
pub enum RorError<T> {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("{0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("{0}")]
    TomlSeError(#[from] toml::ser::Error),
    #[error("{0}")]
    BincodeError(#[from] Box<bincode::ErrorKind>),
    #[error("Error from storage module :{0}")]
    KvError(#[from] KvError),
    #[error("Error from user module :{0}")]
    UserError(#[from] UserError),
    #[error("Datafile Not found :{0}")]
    DataFileNotFound(String),
    #[error("Cannot convert '{0}' to {1}")]
    ConvertError(String,String),
    #[error("Incorrect argument to command '{0}'")]
    ParameterError(String),
    #[error("Unknown type '{0}'")]
    UnknownType(String),
    #[error("Unknown command '{0}'")]
    UnknownCommand(String),
    #[error("{0}")]
    PoisonError(#[from] PoisonError<T>),
    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, RorError<T>>;
