use std::{
    fs,
    io::Read,
    string::String,
    collections::HashMap
};
use serde_json;
use serde::{Deserialize, Serialize};
use super::error::{KvError,Result};

#[derive(Serialize, Deserialize,Clone)]
enum Value {
    Null,
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    HashMap(HashMap<String, Value>),
}

#[derive(Serialize, Deserialize)]
pub enum Command {
    Get = 0,
    Add = 1,
    Delete = 2,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    cmd: Command,
    key: String,
    value: Value,
}

pub struct DataStore {
    path: String,
    data: HashMap<String, Value>,
}

impl DataStore {
    pub fn open(path: String) -> Result<DataStore> {
        todo!()
    }
    fn load(path: String) -> Result<HashMap<String,Value>> {
        let mut file = std::fs::File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let data: Vec<Entry> = serde_json::from_str(&content)?;
        let mut result = HashMap::new();
        for row in &data {
            match row.cmd {
                Command::Add => {
                    todo!()
                },
                Command::Delete => result.remove(&row.key),
                _ => todo!(),
            };
        }
        Ok(result)
    }

    fn add(&mut self, key: String, value: Value) -> Result<()> {
        todo!()
    }
    fn delete(key: String) -> Result<()> {
        todo!()
    }
}