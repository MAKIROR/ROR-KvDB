use std::{
    fs,
    io::{
        self, 
        BufWriter, 
        BufReader,
        Write,
        Read,
    },
    path::PathBuf,
    string::String,
    collections::HashMap,
    fs::File,
};
use super::error::{KvError,Result};
use bincode;
use bytesize::ByteSize;

pub enum Command {
    Get,
    Add,
    Delete,
}

pub struct Entry {
    command: Command,
    key: String,
    value: Vec<u8>,
}

impl Entry {
    pub fn new<T: Sized + serde::Serialize>(command: Command, key: String, v: T) -> Entry {
        Entry {
            command,
            key,
            value: bincode::serialize(&v).unwrap(),
        }
    }
}

pub struct DataStore {
    path: String,
    file_size: u64,
    file_reader: BufReader<File>,
    file_writer: BufWriter<File>,
    data: HashMap<String, u64>,
}

impl DataStore {
    pub fn open(p: String) -> Result<DataStore> {
        let file_size = ByteSize::mb(1).as_u64();
        let created_dir = fs::create_dir_all(&p);
        if let Err(err) = created_dir {
            return Err(KvError::IOError(err));
        }    
        let mut file_reader = BufReader::new(File::open(&p)?);
        let mut file_writer = BufWriter::new(File::create(&p)?);
        let mut result = DataStore {
            path: p,
            file_size,
            file_reader,
            file_writer,
            data: HashMap::new(),
        };
        result.load();
        Ok(result) 
    }
    fn load(&mut self) -> Result<()> {
        let mut offset = 0;
        todo!()
    }
    fn add(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        todo!()
    }
    fn delete(key: &[u8]) -> Result<()> {
        todo!()
    }
    fn write(&mut self, entry: Entry) -> Result<()> {
        todo!()
    }
}