use std::{
    fs,
    io::Read,
    path::PathBuf,
    string::String,
    collections::HashMap
};
use serde_json;
use serde::{Deserialize, Serialize};
use super::error::{KvError,Result};

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

impl Entry {
    pub fn encode(&self) -> Vec<u8> {
    }
    pub fn decode(&self) -> Result<Entry> {
    }
}

pub struct DataStore {
    path: String,
    file_size: u64,
    data: HashMap<String, u64>,
}

impl DataStore {
    pub fn open(path: String) -> Result<DataStore> {
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