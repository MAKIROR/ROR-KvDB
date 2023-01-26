use super::{
    store::kv::{Value,USIZE_SIZE},
    user::{
        user::User,
        user_error::UserError,
    },
    error::{RorError,Result},
};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectRequest {
    pub db_path: String,
    pub user_name: String,
    pub password: String,
}


impl ConnectRequest {
    pub fn new(db_path: String, user_name: String, password: String) -> Self {
        Self {
            db_path: db_path,
            user_name: user_name,
            password: password,
        }
    }
    pub fn as_bytes(&self) -> Result<(Vec<u8>, usize)> {
        let body_buf = bincode::serialize(&self)?;
        let body_size = body_buf.len();
        let size = USIZE_SIZE + body_size;
        let mut buf = vec![0; size];
        buf[0..USIZE_SIZE].copy_from_slice(&body_size.to_be_bytes());
        buf[USIZE_SIZE..].copy_from_slice(&body_buf);
        Ok((buf, body_size))
    }
}

#[derive(Serialize, Deserialize)]
pub enum ConnectReply {
    Success(User),
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
    Quit,
}

#[derive(Serialize, Deserialize)]
pub enum OperateResult {
    Success(Value),
    PermissionDenied,
    Failure,
}