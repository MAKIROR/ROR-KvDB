use std::{
    fs::File,
    io::{
        Error,Write
    },
    path::PathBuf
}
use std::collections::{BTreeMap, HashMap};
use serde::{Serialize, Deserialize}; 

pub enum Command {
    Add { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

impl Command {
    fn add( key: String,value: String ) -> Command {
        Command::Add { key,value }
    }
    fn get( key: String ) -> Command {
        Command::Get { key }
    }
    fn remove( key: String ) -> Command {
        Command::Remove { key }
    }
}

pub struct DataStore {
    path: PathBuf,
    rows: Vec<String>
}

impl DataStore {
    pub fn Open( path : String ) -> Result<Self, Error>  {
        let contents = fs::read_to_string(path)?;
        let mut row = Vec::new();
        for value in contents.lines() {
            rows.push(value);
        }
        Ok(Self {
            path,
            rows: row,
        })
    }
    pub fn CreateFile( path: String ) -> Result<(), Error>  {
        let file_path: Vec<&str> = path.split("/").collect();
        if file_path.len() > 1 {
            let mut dir = String::new();
            for i in 0..file_path.len() - 1 {
                dir.push_str(file_path[i]);
                dir.push_str("/");
            }
            std::fs::create_dir_all(dir).unwrap();
        }
        let mut file = fs::File::create(path)?;
        Ok(());
    }
    pub fn Get(key: String) -> Result<String> {

    }
    pub fn Add(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::add(key, value);
    }
}