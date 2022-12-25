extern crate serde;
#[macro_use] 
extern crate quick_error;

pub use kv::DataStore;

pub mod kv;
mod error;