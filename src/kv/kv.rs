use std::{
    fs,
    String,
    io::{
        Error,
        Write
    },
    path::PathBuf,
    collections::{
        BTreeMap, 
        HashMap
    }
}
use serde::{Serialize, Deserialize}; 
use super::error::KvError;

pub enum Command {
    Add,
    Get,
    Delete,
}

pub struct Entry {
    key: String,
    value: String,
    key_size: usize,
    value_size: usize,
    cmd: Command,
}

impl Entry {
    pub fn new(key: String,value: String,cmd: Command) -> Entry {
        Entry {
            key,
            value,
            key_size: key.as_bytes().len(),
            value_size: value.as_bytes().len(),
            cmd,
        }
    }
    pub fn Encode(&self) -> Vec<u8> {
    }
}

pub struct DataStore {
    path: PathBuf,
    index: HashMap<String, u64>,
}

pub trait Operate {
    fn Get(key: String) -> Option<String>;
    fn Add(&mut self, key: String, value: String) -> Result<()>;
    fn Delete(key: String) -> Result<()>;
}

impl Operate for DataStore {
    fn Get(&mut self, key: String) -> Option<String> {
    }
    fn Add(&mut self, key: String, value: String) -> Result<()> {
        let content = Entry::new(key,value,Command::Add);
        self.write(content)?;
        Ok(())
    }
    fn Delete(key: String) -> Result<()> {
    }
}

impl DataStore {
    fn Write(&mut self, entry: Entry) -> Result<()> {
    }
    fn Read(&mut self, key: &str) -> Result<Entry> {
        if Some(pos) = self.index.get(key) {
            //todo:processing position
        } else {
            Err(KvError::KeyNotFound)
        }
    }
}

pub fn CreateFile( path: String ) -> Result<(), Error>  {
    let file_path: Vec<&str> = path.split("/").collect();
    if file_path.len() > 1 {
        let mut dir = String::new();
        for i in 0..file_path.len() - 1 {
            dir.push_str(file_path[i]);
            dir.push_str("/");
        }
        fs::create_dir_all(dir).unwrap();
    }
    let mut file = fs::File::create(path)?;
    Ok(());
}