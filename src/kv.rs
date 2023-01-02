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
const ENTRY_META_SIZE: usize = USIZE_SIZE * 2 + 1;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Command {
    Add = 0,
    Delete = 1,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Entry {
    Add { meta: Meta, key: String, value: Vec<u8> },
    Delete { meta: Meta, key: String },
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
        Entry::Add {
            meta: Meta {
                command: Command::Add,
                key_size: key.as_bytes().len(),
                value: value_u8_array.len()
            },
            key,
            value: value
        }
    }    
    pub fn delete(key: String) -> Entry {
        Entry::Delete {
            meta: Meta {
                command: Command::Add,
                key_size: key.as_bytes().len(),
                value: 0,
            },
            key
        }
    }
    pub fn size(&self) -> usize {
        ENTRY_META_SIZE + self.meta.key_size + self.meta.value_size
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = vec![0; self.size()];
        let value_u8_array: &[u8] = &self.value;
        buf[0..ENTRY_META_SIZE - USIZE_SIZE * 2].copy_from_slice(bincode::serialize(&self.command).unwrap().as_slice());
        buf[ENTRY_META_SIZE - USIZE_SIZE * 2..USIZE_SIZE].copy_from_slice(&self.meta.key_size.to_be_bytes());
        buf[USIZE_SIZE..USIZE_SIZE * 2].copy_from_slice(&self.meta.value_size.to_be_bytes());
        buf[ENTRY_META_SIZE..ENTRY_META_SIZE + self.meta.key_size].copy_from_slice(self.key.as_bytes());
        buf[ENTRY_META_SIZE + self.meta.key_size..].copy_from_slice(value_u8_array);
        Ok(buf)
    }
    pub fn decode(buf: &[u8; ENTRY_META_SIZE]) -> Result<Meta> {
        let command: Command = usize::from_be_bytes(buf[0..ENTRY_META_SIZE - USIZE_SIZE * 2].try_into()?);
        let key_size = usize::from_be_bytes(buf[ENTRY_META_SIZE - USIZE_SIZE * 2..USIZE_SIZE].try_into()?);
        let value_size = usize::from_be_bytes(buf[USIZE_SIZE..USIZE_SIZE * 2].try_into()?);
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
        };
        result.index = result.load_hashmap()?;
        Ok(result)
    }
    pub fn get(&mut self, key: String) -> Result<Option<Vec<u8>>> {
        match self.read(&key) {
            Ok(entry) => return Ok(Some(entry.value)),
            Err(KvError::KeyNotFound(key)) => return Ok(None),
            Err(e) => return Err(e),
        }
    }
    pub fn add<T: Sized + serde::Serialize>(&mut self, key: String, value: T) -> Result<()> {
        let encode_value: Vec<u8> = bincode::serialize(&value)?;
        let entry = Entry::add(key, encode_value,)?;
        self.write(&entry)?;
        Ok(())
    }
    pub fn delete(&mut self, key: String) -> Result<()> {
        self.index.remove(&key);
        let entry = Entry::delete(key);
        self.write(&entry)?;
        Ok(())
    }
    pub fn compact(&mut self) -> Result<()> {
        let new_vec = self.load_vec()?;
        if new_vec.is_empty() {
            return Ok(());
        }
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
                    let size = entry.length() as u64;
                    match entry() {
                        Entry::Add {meta, key, value} => {new_hashmap.insert(*entry.key, offset,);}
                        Entry::Delete {meta, key} => {new_hashmap.remove(&entry.key);}
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
                    let size = entry.length() as u64;
                    if let Some(valid_pos) = self.index.get(&entry.key) {
                        if entry.meta.command == Command::Add {
                            new_vec.push(entry);
                            offset += size;
                        }
                    }
                },
                Err(KvError::EOF) => {break;}
                Err(e) => return Err(e),
            }
        }
        Ok(new_vec)
    }
    fn write(&mut self, entry: &Entry) -> Result<()> {
        let buf = entry.encode()?; 
        self.file_writer.write(&buf)?;
        self.file_writer.flush()?;
        Ok(())
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
            Ok(entry_head) => {
                let mut key_buf = vec![0; entry_head.key_size];
                self.file_reader.read_exact(key_buf.as_mut_slice())?;
                let key = String::from_utf8(key_buf)?;
                
                let mut value_buf = vec![0; entry_head.value_size];
                self.file_reader.read_exact(value_buf.as_mut_slice())?;
                let result: Entry = match entry_head.command {
                    Command::Add => {
                        Entry::Add {
                            meta: Meta {
                                command: Command::Add,
                                key_size: entry_head.key_size,
                                value_size: entry_head.value_size,
                            },
                            key: key,
                            value: value_buf,
                        }
                    }
                    Command::Delete => {
                        Entry::Delete {
                            meta: Meta {
                                command: Command::Delete,
                                key_size: entry_head.key_size,
                                value_size: entry_head.value_size,
                            },
                            key: key,
                        }
                    }
                };
                Ok(result)
            },
            Err(e) => Err(e),
        };
    }
}