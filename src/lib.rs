extern crate serde;

pub use kv::DataStore;
pub use error::{KvError, Result};

pub mod kv;
mod error;