use super::store::kv::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConnectRequest {
    db_path: String,
    user_name: String,
    user_password: String,
}

#[derive(Serialize, Deserialize)]
pub enum OperateRequest {
    Get { key: String },
    Add { key: String, value: Value },
    Delete { key: String },
}