use super::{
    store::kv::Value,
    user::{
        user::User,
        user_error::UserError,
    },
};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ConnectRequest {
    pub db_path: String,
    pub user_name: String,
    pub password: String,
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
}

#[derive(Serialize, Deserialize)]
pub enum OperateResult {
    Success(Value),
    PermissionDenied,
    Failure,
}