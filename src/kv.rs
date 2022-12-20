use std::{
<<<<<<< HEAD
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
=======
    fs::File,
    io::{
        Error,Write
    },
    path::PathBuf
>>>>>>> aca9033024fb77cd5f523f9d4dc4df674e4e66fe
}
use std::collections::{BTreeMap, HashMap};
use serde::{Serialize, Deserialize}; 

pub enum Command {
<<<<<<< HEAD
    Add { key: String, value: String ,},
    Get { key: String },
    Delete { key: String },
=======
    Add { key: String, value: String },
    Get { key: String },
    Remove { key: String },
>>>>>>> aca9033024fb77cd5f523f9d4dc4df674e4e66fe
}

impl Command {
    fn add( key: String,value: String ) -> Command {
        Command::Add { key,value }
    }
    fn get( key: String ) -> Command {
        Command::Get { key }
    }
<<<<<<< HEAD
    fn delete( key: String ) -> Command {
=======
    fn remove( key: String ) -> Command {
>>>>>>> aca9033024fb77cd5f523f9d4dc4df674e4e66fe
        Command::Remove { key }
    }
}

pub struct DataStore {
    path: PathBuf,
<<<<<<< HEAD
    data: Vec<Entry>,
}

pub struct Entry {
    key: String,
    value: String,
    type: Command,
}

impl DataStore {
=======
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
>>>>>>> aca9033024fb77cd5f523f9d4dc4df674e4e66fe
    pub fn Get(key: String) -> Result<String> {

    }
    pub fn Add(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::add(key, value);
<<<<<<< HEAD
        
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
=======
    }
>>>>>>> aca9033024fb77cd5f523f9d4dc4df674e4e66fe
}