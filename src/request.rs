use super::{
    store::kv::Value,
    error::Result,
};
use serde::{Serialize,Deserialize,de::DeserializeOwned};

pub const USIZE_SIZE: usize = std::mem::size_of::<usize>();

#[derive(Serialize, Deserialize)]
pub struct ConnectRequest {
    pub db_path: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub enum ConnectReply {
    Success,
    Error(ConnectError),
}

#[derive(Serialize, Deserialize)]
pub enum ConnectError {
    RequestError,
    UserNotFound,
    PasswordError,
    OpenFileError,
    PathError,
    ServerError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OperateRequest {
    Open { path: String },
    Get { key: String },
    Add { key: String, value: Value },
    Delete { key: String },
    CreateUser { name: String, password: String, level: String },
    DeleteUser { name: String },
    GetType { key: String },
    Compact,
    Quit,
}

#[derive(Serialize, Deserialize)]
pub enum OperateResult {
    Found(Value),
    Type(String),
    Success,
    PermissionDenied,
    KeyNotFound,
    Failure,
}

pub struct Message<T> { 
    message: T,
}

impl<T: Serialize + DeserializeOwned> Message<T>{
    pub fn new(message: T) -> Self {
        Self {
            message
        }
    }
    pub fn as_bytes(&self) -> Result<(Vec<u8>, usize)> {
        let body_buf = bincode::serialize(&self.message)?;
        let body_size = body_buf.len();
        let size = USIZE_SIZE + body_size;
        let mut buf = vec![0; size];
        buf[0..USIZE_SIZE].copy_from_slice(&body_size.to_be_bytes());
        buf[USIZE_SIZE..].copy_from_slice(&body_buf);
        Ok((buf, body_size))
    }
}