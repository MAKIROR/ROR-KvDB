use super::store::kv::Value;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ConnectRequest {
    pub db_path: String,
    pub user_name: String,
    pub user_password: String,
}

#[derive(Serialize, Deserialize)]
pub enum ConnectReply {
    Success,
    Error(ConnectError),
}

#[derive(Serialize, Deserialize)]
pub enum ConnectError {
    RequestError,
    UserError,
    FileError,
    OpenFileError,
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
    Failure,
}