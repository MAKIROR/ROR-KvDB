use super::{
    store::kv::{Value,USIZE_SIZE},
    error::Result,
};
use serde::{Serialize,Deserialize,de::DeserializeOwned};

#[derive(Serialize, Deserialize, Debug)]
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
    UserNotFound(String),
    PasswordError,
    OpenFileError,
    PathError,
    ServerError,
}

#[derive(Serialize, Deserialize)]
pub enum OperateRequest {
    Get { key: String },
    Add { key: String, value: Value },
    Delete { key: String },
    Compact,
    Quit,
}

#[derive(Serialize, Deserialize)]
pub enum OperateResult {
    Success(Value),
    PermissionDenied,
    KeyNotFound(String),
    Failure,
}

pub struct Message<T> { 
    message: T,
}

impl<T: Serialize + DeserializeOwned> Message<T>{
    pub fn new(message: T) -> Message<T> {
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