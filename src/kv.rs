use std::{
    fs,
    io::{
        self, 
        BufWriter, 
        BufReader,
        Write,
        Read,
        Seek,
        SeekFrom,
    },
    path::PathBuf,
    string::String,
    collections::HashMap,
    fs::File,
};
use super::error::{KvError,Result};
use bytesize::ByteSize;
use bincode;
use serde;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Command {
    Get,
    Add,
    Delete,
}

#[derive(serde::Serialize, serde::Deserialize)]
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
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        buf = bincode::serialize(&self)?;
        Ok(buf)
    }
    pub fn decode(buf: &[u8]) -> Result<Entry> {
        let result: Entry = bincode::deserialize(&buf)?;
        Ok(result)
    }
}

pub struct DataStore {
    path: String,
    file_size: u64,
    file_reader: BufReader<File>,
    file_writer: BufWriter<File>,
    index: HashMap<String, EntryPos>,
}

pub struct EntryPos {
    position: u64,
    length: u64,
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
            index: HashMap::new(),
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
        let buf = entry.encode()?; 
        self.file_writer.write(&buf)?;
        self.file_writer.flush()?;
        Ok(())
    }
    fn read(&mut self, key: &str) -> Result<Entry> {
        if let Some(entry_pos) = self.index.get(key) {
            let position = entry_pos.position;
            self.file_reader.seek(SeekFrom::Start(position))?;
            let mut entry_buf = [0; entry_pos.length];
            if self.file_reader.read(&mut entry_buf)? == 0 {
                return Err(KvError::EOF);
            }
            match Entry::decode(&entry_buf) {
                Ok(entry) => Ok(entry),
                Err(e) => Err(e),
            }
        }
    }
}