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

const USIZE_SIZE: usize = std::mem::size_of::<usize>();
const META_SIZE: usize = 2 * USIZE_SIZE + 1;
const COMPACTION_THRESHOLD: u64 = 1024*1024*1024;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Command {
    Get,
    Add,
    Delete,
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
    pub fn new<T: Sized + serde::Serialize>(command: Command, key: String, v: T) -> Result<Entry> {
        let value: Vec<u8> = bincode::serialize(&v)?;
        Ok(Entry {
            meta: Meta {
                command,
                key_size: key.as_bytes().len(),
                value_size: value.len(),
            },
            key,
            value: value,
        })
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut entry_buf = vec![0;self.length()];
        let value_u8: &[u8] = &self.value;
        entry_buf[0..USIZE_SIZE].copy_from_slice(&self.meta.key_size.to_be_bytes());
        entry_buf[USIZE_SIZE..USIZE_SIZE * 2].copy_from_slice(&self.meta.value_size.to_be_bytes());
        entry_buf[USIZE_SIZE * 2..META_SIZE].copy_from_slice(bincode::serialize(&self.meta.command)?.as_slice());
        entry_buf[META_SIZE..META_SIZE + self.meta.key_size].copy_from_slice(self.key.as_bytes());
        entry_buf[META_SIZE + self.meta.key_size..].copy_from_slice(value_u8);
        Ok(entry_buf)
    }
    pub fn decode(buf: &[u8; META_SIZE]) -> Result<Meta> {
        let key_size = usize::from_be_bytes(buf[0..USIZE_SIZE].try_into()?);
        let value_size = usize::from_be_bytes(buf[USIZE_SIZE..USIZE_SIZE * 2].try_into()?);
        let command: Command = bincode::deserialize(&buf[USIZE_SIZE * 2..META_SIZE])?;

        Ok(Meta {
            key_size,
            value_size,
            command,
        })
    }
    pub fn length(&self) -> usize {
        META_SIZE + self.meta.key_size + self.meta.value_size
    }
}

pub struct DataStore {
    path: String,
    file_size: u64,
    file_reader: BufReader<File>,
    file_writer: BufWriter<File>,
    position: u64,
    index: HashMap<String, u64>,
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
            position: 0,
            index: HashMap::new(),
        };
        result.load();
        Ok(result)
    }
    fn load(&mut self) -> Result<()> {
        let mut offset = 0;
        loop {
            match self.read_with_offset(offset) {
                Ok(entry) => {
                    match entry.meta.command {
                        Command::Add => {
                            let size = entry.length() as u64;
                            offset += size;
                            self.index.insert(entry.key, offset,);
                        }
                        Command::Delete => {
                            self.index.remove(&entry.key);
                        }
                        _ => continue,
                    }
                },
                Err(KvError::EOF) => {
                    self.position = offset;
                    return Ok(());
                },
                Err(e) => return Err(e),
            }
        }
    }
    fn add(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        todo!()
    }
    fn delete(key: &String) -> Result<()> {
        todo!()
    }
    pub fn compact(&mut self) -> Result<()> {
        let mut offset = 0;
        let mut new_hashmap: HashMap<String, u64> = HashMap::new();
        loop {
            match self.read_with_offset(offset) {
                Ok(entry) => {
                    match entry.meta.command {
                        Command::Add => {
                            let size = entry.length() as u64;
                            offset += size;
                            new_hashmap.insert(entry.key, offset,);
                        }
                        Command::Delete => {
                            new_hashmap.remove(&entry.key);
                        }
                        _ => continue,
                    }
                },
                Err(KvError::EOF) => {break;}
                Err(e) => return Err(e),
            }
        }
        self.index = new_hashmap;
        Ok(())
    }
    fn write(&mut self, entry: Entry) -> Result<()> {
        let buf = entry.encode()?; 
        self.file_writer.write(&buf)?;
        self.file_writer.flush()?;
        Ok(())
    }
    fn read(&mut self, key: &str) -> Result<Entry> {
        if let Some(entry_pos) = self.index.get(key) {
            return self.read_with_offset(*entry_pos);
        }
        Err(KvError::KeyNotFound(key.to_string()))
    }
    fn read_with_offset(&mut self, offset: u64) -> Result<Entry> {
        self.file_reader.seek(SeekFrom::Start(offset))?;
        let mut entry_buf: [u8; META_SIZE] = [0; META_SIZE];
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

                Ok(Entry {
                    meta: Meta{
                        command: entry_meta.command,
                        key_size: entry_meta.key_size,
                        value_size: entry_meta.key_size,
                    },
                    key: key,
                    value: value_buf,
                })
            },
            Err(e) => Err(e),
        };
    }
}