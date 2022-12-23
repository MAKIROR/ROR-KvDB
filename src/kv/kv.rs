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

pub enum Command<T> {
    Add { key: String, value: T },
    Get { key: String },
    Delete {key: String },
}

pub struct DataStore {
    path: PathBuf,
    index: HashMap<String, Command>,
}

impl<T> DataStore<T> {
    fn Get(&mut self, key: String) -> Result<Option<T>> {
        match self.index.get(&key) { 
            None => Err(KvError::KeyNotFound),
            Some(c) => {
                Ok(Some(c.value)),
            }
        }
    }
    fn Add(&mut self, key: String, value: String) -> Result<()> {
        let content = Command::Add(key,value);
        self.write(content)?;
        Ok(())
    }
    fn Delete(key: String) -> Result<()> {

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