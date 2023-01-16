pub use store::kv::{DataStore,Value,Command};
pub use store::kv_error::{KvError, Result};
//pub use service::{Server,Config};
pub use user::user::User;

mod store;
//mod service;
mod user;