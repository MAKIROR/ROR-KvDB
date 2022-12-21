use std::io;
use failure::Fail;

#[derive(Fail, Debug)]
pub enum KvError {
    #[fail(display = "Invalid Path")]
    InvalidPath,
    #[fail(display = "Key not found")]
    KeyNotFound,

}