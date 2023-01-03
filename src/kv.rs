use std::{
    fs::{self,OpenOptions},
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

const USIZE_SIZE: usize = std::mem::size_of::<usize>();
const ENTRY_META_SIZE: usize = USIZE_SIZE * 2 + 4;

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Command {
    Add = 0,
    Delete = 1,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Entry {
    meta: Meta, 
    key: String, 
    value: Vec<u8>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Meta {
    command: Command,
    key_size: usize,
    value_size: usize,
}

impl Entry {
    pub fn add(key: String, value: Vec<u8>) -> Entry {
        let value_u8_array: &[u8] = &value;
        Entry {
            meta: Meta {
                command: Command::Add,
                key_size: key.as_bytes().len(),
                value_size: value_u8_array.len()
            },
            key,
            value: value
        }
    }    
    pub fn delete(key: String) -> Entry {
        Entry {
            meta: Meta {
                command: Command::Add,
                key_size: key.as_bytes().len(),
                value_size: 0,
            },
            key,
            value: vec![0; 0]
        }
    }
    pub fn size(&self) -> usize {
        ENTRY_META_SIZE + self.meta.key_size + self.meta.value_size
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = vec![0; self.size()];
        let value_u8_array: &[u8] = &self.value;
        buf[0..ENTRY_META_SIZE - USIZE_SIZE * 2].copy_from_slice(bincode::serialize(&self.meta.command).unwrap().as_slice());
        buf[ENTRY_META_SIZE - USIZE_SIZE * 2..ENTRY_META_SIZE - USIZE_SIZE].copy_from_slice(&self.meta.key_size.to_be_bytes());
        buf[ENTRY_META_SIZE - USIZE_SIZE..ENTRY_META_SIZE].copy_from_slice(&self.meta.value_size.to_be_bytes());
        buf[ENTRY_META_SIZE..ENTRY_META_SIZE + self.meta.key_size].copy_from_slice(self.key.as_bytes());
        buf[ENTRY_META_SIZE + self.meta.key_size..].copy_from_slice(value_u8_array);
        Ok(buf)
    }
    pub fn decode(buf: &[u8; ENTRY_META_SIZE]) -> Result<Meta> {
        let command: Command = bincode::deserialize(&buf[0..ENTRY_META_SIZE - USIZE_SIZE * 2])?;
        let key_size = usize::from_be_bytes(buf[ENTRY_META_SIZE - USIZE_SIZE * 2..ENTRY_META_SIZE - USIZE_SIZE].try_into()?);
        let value_size = usize::from_be_bytes(buf[ENTRY_META_SIZE - USIZE_SIZE..ENTRY_META_SIZE].try_into()?);
        Ok(
            Meta{
                command,
                key_size,
                value_size,
            }
        )
    }
}

pub struct DataStore {
    path: String,
    file_size: u64,
    file_reader: BufReader<File>,
    file_writer: BufWriter<File>,
    index: HashMap<String, u64>,
    position: u64,
    uncompacted: u64,
}

impl DataStore {
    pub fn open(p: String) -> Result<DataStore> {
        let file_size = ByteSize::mb(1).as_u64();
        let mut file_reader = BufReader::new(File::open(&p)?);
        let mut file_writer = BufWriter::new(OpenOptions::new().create(true).write(true).append(true).open(&p)?);
        let mut result = DataStore {
            path: p,
            file_size,
            file_reader,
            file_writer,
            index: HashMap::new(),
            position: 0,
            uncompacted: 0,
        };
        result.index = result.load_hashmap()?;
        Ok(result)
    }
    pub fn get(&mut self, key: String) -> Result<Vec<u8>> {
        match self.read(&key) {
            Ok(entry) => return Ok(entry.value),
            Err(KvError::KeyNotFound(key)) => Err(KvError::KeyNotFound(key)),
            Err(e) => return Err(e),
        }
    }
    pub fn add<T: Sized + serde::Serialize>(&mut self, key: String, value: T) -> Result<()> {
        let encode_value: Vec<u8> = bincode::serialize(&value)?;
        let entry = Entry::add((*key).to_string(), encode_value,);
        let size = self.write(&entry)? as u64;
        if let Some(_pos) = self.index.get(&key){
            self.uncompacted += size;
        }
        Ok(())
    }
    pub fn delete(&mut self, key: String) -> Result<()> {
        if let Some(_pos) = self.index.get(&key){
            self.index.remove(&key);
            let entry = Entry::delete(key);
            let size = self.write(&entry)?;
            self.uncompacted += size;
            return Ok(());
        }
        Err(KvError::KeyNotFound(key))
    }
    pub fn compact(&mut self) -> Result<()> {
        let new_vec = self.load_vec()?;
        if new_vec.is_empty() {
            return Ok(());
        }
        fs::remove_file(&self.path)?;
        self.file_writer = BufWriter::new(File::create(&self.path)?);
        self.file_reader = BufReader::new(File::open(&self.path)?);
        for entry in &new_vec {
            self.write(&entry);
        }
        Ok(())
    }
    fn load_hashmap(&mut self) -> Result<HashMap<String, u64>> {
        let mut offset = 0;
        let mut new_hashmap: HashMap<String, u64> = HashMap::new();
        loop {
            match self.read_with_offset(offset) {
                Ok(entry) => {
                    let size = entry.size() as u64;
                    match entry.meta.command {
                        Command::Add => {new_hashmap.insert((*entry.key).to_string(), offset,);}
                        Command::Delete => {new_hashmap.remove(&entry.key);}
                    }
                    offset += size;
                },
                Err(KvError::EOF) => {break;}
                Err(e) => return Err(e),
            }
        }
        Ok(new_hashmap)
    }
    fn load_vec(&mut self) -> Result<Vec<Entry>> {
        self.index = self.load_hashmap()?;
        let mut offset = 0;
        let mut new_vec: Vec<Entry> = Vec::new();
        loop {
            match self.read_with_offset(offset) {
                Ok(entry) => {
                    let size = entry.size() as u64;
                    offset += size;
                    if entry.meta.command != Command::Add {
                        continue;
                    }
                    if let Some(valid_pos) = self.index.get(&entry.key){
                        new_vec.push(entry);
                    }
                },
                Err(KvError::EOF) => {break;}
                Err(e) => return Err(e),
            }
        }
        Ok(new_vec)
    }
    fn write(&mut self, entry: &Entry) -> Result<u64> {
        let buf = entry.encode()?; 
        let size = buf.len() as u64;
        self.position += size;
        self.file_writer.write(&buf)?;
        self.file_writer.flush()?;
        Ok(size)
    }
    fn read(&mut self, key: &String) -> Result<Entry> {
        if let Some(offset) = self.index.get(key) {
            return self.read_with_offset(*offset);
        }
        Err(KvError::KeyNotFound(key.to_string()))
    }
    fn read_with_offset(&mut self, offset: u64) -> Result<Entry> {
        self.file_reader.seek(SeekFrom::Start(offset))?;
        let mut entry_buf: [u8; ENTRY_META_SIZE] = [0; ENTRY_META_SIZE];
        let len = self.file_reader.read(&mut entry_buf)?;
        if len == 0 {
            return Err(KvError::EOF);
        }
        return match Entry::decode(&entry_buf) {
            Ok(entry_meta) => {
                let mut key_buf = vec![0; entry_meta.key_size];
                self.file_reader.read_exact(key_buf.as_mut_slice())?;
                let key = String::from_utf8(key_buf)?;
                
                let mut value_buf = vec![0; entry_meta.value_size];
                self.file_reader.read_exact(value_buf.as_mut_slice())?;
                let result: Entry = match entry_meta.command {
                    Command::Add => {
                        Entry {
                            meta: Meta {
                                command: Command::Add,
                                key_size: entry_meta.key_size,
                                value_size: entry_meta.value_size,
                            },
                            key: key,
                            value: value_buf,
                        }
                    }
                    Command::Delete => {
                        Entry {
                            meta: Meta {
                                command: Command::Delete,
                                key_size: entry_meta.key_size,
                                value_size: entry_meta.value_size,
                            },
                            key: key,
                            value: vec![0; 0],
                        }
                    }
                };
                Ok(result)
            },
            Err(e) => Err(e),
        };
    }
}