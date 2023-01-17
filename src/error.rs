use thiserror::Error;
use super::{
    store::kv_error::KvError,
    user::user_error::UserError,
};

#[derive(Error, Debug)]
pub enum RorError {
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
    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, RorError>;
