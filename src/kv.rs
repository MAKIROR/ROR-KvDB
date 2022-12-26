use std::{
    fs,
    io::Read,
    string::String,
    collections::HashMap
};
use serde_json::Result;
use super::error::KvError;

enum Value {
    Null,
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    HashMap(HashMap<String, Value>),
}

pub enum Command {
    Add,
    Get,
    Delete,
}

pub struct Entry {
    cmd: Command,
    key: String,
    value: Value,
}

impl Entry {
    pub fn new(cmd: Command,key: String, value: Value) -> Entry {
        Entry {
            cmd,
            key,
            value,
        }
    }
}

pub struct DataStore {
    path: String,
    data: HashMap<String, Value>,
}

impl DataStore {
    pub fn open(path: String) -> Result<DataStore> {
        let data = Self::load(path);
        match data {
            Ok(result) => Ok(DataStore{
                path,
                data: result
            }),
            Err(e) => Err(e),
        }
    }
    fn load(path: String) -> Result<HashMap<String,Value>> {
        let mut file = std::fs::File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let data: Vec<String> = serde_json::from_str(&content)?;
        let result = HashMap::new();
        for row in &data {
            let r = serde_json::from_str(row)?;
            match r.cmd {
                Command::Add => result.insert(r.key,r.value),
                Command::Delete => result.remove(&r.key),
            };
        }
        Ok(result)
    }

    fn add(&mut self, key: String, value: Value) -> Result<()> {
        let content = Entry::new(
            Command::Add,
            key,
            value,
        );
        //todo
        Ok(())
    }
    fn delete(key: String) -> Result<()> {
        Ok(())
    }
}