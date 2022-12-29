use std::{
    fs,
    io::Read,
    path::PathBuf,
    string::String,
    collections::HashMap
};
use serde_json;
use super::error::{KvError,Result};
use bincode;
use bytesize::ByteSize;

pub enum Command {
    Get,
    Add,
    Delete,
}

pub struct Entry {
    command: Command,
    key_size: usize,
    value_size: usize,
    key: Vec<u8>,
    value: Vec<u8>,
}

pub struct DataStore {
    path: String,
    file_size: u64,
    data: HashMap<String, u64>,
}

impl DataStore {
    pub fn open(p: String, fs: Option<u64>) -> Result<DataStore> {
        let file_size = match fs {
            Some(i) => i,
            None => ByteSize::mb(1).as_u64(),
        }
        todo!()
    }
    fn add(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        todo!()
    }
    fn delete(key: &[u8]) -> Result<()> {
        todo!()
    }
    fn write(&mut self, entry: Entry) -> Result<()> {
        todo!()
    }
}