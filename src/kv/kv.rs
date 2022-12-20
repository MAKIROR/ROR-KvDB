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

pub enum Command {
    Add,
    Get,
    Delete,
}

pub struct DataStore {
    path: PathBuf,
    data: Vec<Entry>,
}

impl DataStore {
    pub fn Get(key: String) -> Result<String> {
    }
    pub fn Add(&mut self, key: String, value: String) -> Result<()> {
        let content = Entry::new(key,value,Command::Add);
        self.write(content)?;
        Ok(())
    }
    pub fn Delete(key: String) -> Result<()> {
    }

    fn write(&mut self, entry: Entry) -> Result<()> {
        
    }
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