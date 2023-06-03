use thiserror::Error;
use super::{
    store::kv_error::KvError,
    user::user_error::UserError,
    request::OperateRequest,
    cmd::{
        cmd_error::CmdError,
        statement::ValueType
    },
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
    #[error("{0}")]
    KvError(#[from] KvError),
    #[error("{0}")]
    UserError(#[from] UserError),
    #[error("{0}")]
    CmdError(#[from] CmdError),

    #[error("Datafile Not found :{0}")]
    DataFileNotFound(String),
    #[error("Cannot convert '{0}' to {1}")]
    ConvertError(String, ValueType),
    #[error("Incorrect argument to command '{0}'")]
    ParameterError(String),
    #[error("Unknown type '{0}'")]
    UnknownType(String),
    #[error("Unknown command '{0}'")]
    UnknownCommand(String),

    #[error("The client actively disconnected")]
    Disconnect,

    #[error("Unable to connect to server: {0}")]
    ConnectFailed(std::io::Error),
    #[error("Server cannot to open datafile")]
    OpenFileFailed,
    #[error("Server cannot parse the request correctly")]
    RequestError,
    #[error("Server cannot parse the path correctly")]
    PathError,
    #[error("Server encountered an unexpected error")]
    ServerError,
    #[error("Unable to communicate with the server, the connection may be interrupted, you can try to reconnect or check the server")]
    ConnectionLost(OperateRequest),
    #[error("nable to communicate with server, possibly high latency or lost connection, please try this operation again")]
    AbnormalConnection,
    #[error("Unable to parse data, probably it is incomplete")]
    IncompleteData,
}

pub type Result<T> = std::result::Result<T, RorError>;
