use std::{
    fs,
    io::Read,
    string::String,
    collections::HashMap
};
use serde_json::Result;
use super::error::KvError;

pub enum Command {
    Add,
    Get,
    Delete,
}

pub struct Entry<T> {
    cmd: Command,
    key: String,
    value: T,
}

impl<T> Entry<T> {
    pub fn new(cmd: Command,key: String, value: T) -> Entry<T> {
        Entry {
            cmd,
            key,
            value,
        }
    }
}

pub struct DataStore<T> {
    path: String,
    data: HashMap<String,T>,
}

impl<T> DataStore<T> {
    pub fn Refresh(&mut self) -> Result<()> {
        let mut file = std::fs::File::open(&self.path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        //todo
        Ok(())
    }
    fn Add(&mut self, key: String, value: T) -> Result<()> {
        let content = Entry::new(
            Command::Add,
            key,
            value,
        );
        //todo
        Ok(())
    }
    fn Delete(key: String) -> Result<()> {
        Ok(())
    }
}

pub fn Open(path: String) -> Result<DataStore> {
    let mut file = std::fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    //todo
    Ok(())
}