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
use serde_json; 
use super::error::KvError;

pub enum Command<T> {
    Add { key: String, value: T },
    Get { key: String },
    Delete {key: String },
}

pub struct DataStore<T> {
    path: String,
    data: HashMap<String,T>,
}

impl<T> DataStore<T> {
    fn Refresh(&mut self) -> Result<(), KvError> {
        let content = match fs::read_to_string(path) {
            Some(c) => c,
            Err(e) => return Err(KvError::IoError(e)),
        }
        content = content.unwrap();
        let jsondata = match serde_json::from_str(&content) {
            Some(j) => j,
            Err(e) => return Err(KvError::SerdeError(e)),
        }
        //todo
    }
    fn Get(&mut self, key: String) -> Result<Option<T>, KvError> {
        match self.data.get(&key) { 
            None => Err(KvError::KeyNotFound(key)),
            Some(c) => {
                Ok(Some(c.value)),
            }
        }
    }
    fn Add(&mut self, key: String, value: String) -> Result<(), KvError> {
        let content = Command::Add(key,value);
        self.write(content)?;
        Ok(())
    }
    fn Delete(key: String) -> Result<()> {
    }
}

pub fn CreateFile( path: String ) -> Result<(), KvError>  {
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