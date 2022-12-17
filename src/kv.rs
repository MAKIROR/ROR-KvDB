use std::{
    fs,
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
use std::collections::{BTreeMap, HashMap};
use serde::{Serialize, Deserialize}; 

pub enum Command {
    Add { key: String, value: String ,},
    Get { key: String },
    Delete { key: String },
}

impl Command {
    fn add( key: String,value: String ) -> Command {
        Command::Add { key,value }
    }
    fn get( key: String ) -> Command {
        Command::Get { key }
    }
    fn delete( key: String ) -> Command {
        Command::Remove { key }
    }
}

pub struct DataStore {
    path: PathBuf,
    data: Vec<Entry>,
}

pub struct Entry {
    key: String,
    value: String,
    type: Command,
}

impl DataStore {
    pub fn Get(key: String) -> Result<String> {

    }
    pub fn Add(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::add(key, value);
        
    }
    pub fn Delete(key: String) -> Result<()> {

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