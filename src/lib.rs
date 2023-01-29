use store::kv::{DataStore,Value,Command,USIZE_SIZE};
use store::kv_error::{KvError, Result};
use user::user::User;
use request::*;
pub use repl::{RemoteRepl,LocalRepl};
pub use server::Server;

mod store;
mod user;
mod server;
mod client;
mod request;
mod error;
mod repl;