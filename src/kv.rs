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
        let mut file = std::fs::File::open(self.path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content = content.unwrap();
        println!("{}",content);
        Ok(())
    }
    fn Get(&mut self, key: String) -> Result<Option<T>> {
        match self.data.get(&key) { 
            None => Err(KvError::KeyNotFound(key)),
            Some(c) => Ok(Some(c.value)),
        }
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

pub fn CreateFile( path: String ) -> Result<()>  {
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
    Ok(())
}