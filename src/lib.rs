extern crate serde;

pub use kv::{DataStore,Entry};
pub use error::{KvError, Result};

pub mod kv;
mod error;