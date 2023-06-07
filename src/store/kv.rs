use std::{
    fmt,
    io::{
        BufWriter, 
        BufReader,
        Write,
        Read,
        Seek,
        SeekFrom,
    },
    string::String,
    collections::HashMap,
    fs::{self, File,OpenOptions},
};
use bincode;
use serde::{Serialize,Deserialize};
use super::kv_error::{KvError,Result};

const USIZE_SIZE: usize = std::mem::size_of::<usize>();
const ENTRY_META_SIZE: usize = USIZE_SIZE * 2 + 4;
const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

#[derive(Serialize, Deserialize, PartialEq,Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    String(String),
    Char(char),
    Array(Box<Vec<Value>>)
}

#[derive(Serialize, Deserialize, PartialEq,Debug)]
pub enum Command {
    Add,
    Delete,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Entry {
    meta: Meta, 
    key: String, 
    value: Value,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Meta {
    command: Command,
    key_size: usize,
    value_size: usize,
}

impl Entry { 
    pub fn add(key: String, value: Value, value_size: usize) -> Entry {
        Entry {
            meta: Meta {
                command: Command::Add,
                key_size: key.as_bytes().len(),
                value_size: value_size
            },
            key,
            value: value
        }
    }   
    pub fn delete(key: String) -> Entry {
        Entry {
            meta: Meta {
                command: Command::Delete,
                key_size: key.as_bytes().len(),
                value_size: 4,
            },
            key,
            value: Value::Null,
        }
    }
    pub fn size(&self) -> usize {
        ENTRY_META_SIZE + self.meta.key_size + self.meta.value_size
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = vec![0; self.size()];
        buf[0..ENTRY_META_SIZE - USIZE_SIZE * 2].copy_from_slice(bincode::serialize(&self.meta.command)?.as_slice());
        buf[ENTRY_META_SIZE - USIZE_SIZE * 2..ENTRY_META_SIZE - USIZE_SIZE].copy_from_slice(&self.meta.key_size.to_be_bytes());
        buf[ENTRY_META_SIZE - USIZE_SIZE..ENTRY_META_SIZE].copy_from_slice(&self.meta.value_size.to_be_bytes());
        buf[ENTRY_META_SIZE..ENTRY_META_SIZE + self.meta.key_size].copy_from_slice(self.key.as_bytes());
        buf[ENTRY_META_SIZE + self.meta.key_size..].copy_from_slice(bincode::serialize(&self.value)?.as_slice());
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

#[derive(Debug)]
pub struct DataStore {
    pub path: String,
    file_reader: BufReader<File>,
    file_writer: BufWriter<File>,
    index: HashMap<String, u64>,
    position: u64,
    uncompacted: u64,
}

impl DataStore {
    pub fn open(path: &str) -> Result<DataStore> {
        let file_writer = BufWriter::new(OpenOptions::new().write(true).append(true).create(true).open(path)?);
        let file_reader = BufReader::new(File::open(path)?);
        let mut result = DataStore {
            path: path.to_string(),
            file_reader,
            file_writer,
            index: HashMap::new(),
            position: 0,
            uncompacted: 0,
        };
        (result.index, result.uncompacted) = result.load_hashmap()?;
        Ok(result)
    }

    pub fn get(&mut self, key: String) -> Result<Value> {
        match self.read(&key) {
            Ok(entry) => {
                return Ok(entry.value);
            },
            Err(KvError::KeyNotFound(key)) => Err(KvError::KeyNotFound(key)),
            Err(e) => return Err(e),
        }
    }

    pub fn get_all_value(&mut self) -> Result<Vec<Value>> {
        let mut data: Vec<Value> = Vec::new();
        let mut offset_vec: Vec<u64> = Vec::new();
        for (_, offset) in &self.index {
            offset_vec.push(*offset);
        }
        for offset in offset_vec {
            let value = match self.read_with_offset(offset.clone()) {
                Ok(entry) => entry.value,
                Err(e) => return Err(e),
            };
            data.push(value);
        }
        return Ok(data);
    }

    pub fn get_all_entry(&mut self) -> Result<Vec<Entry>> {
        let mut data: Vec<Entry> = Vec::new();
        let mut offset_vec: Vec<u64> = Vec::new();
        for (_, offset) in &self.index {
            offset_vec.push(*offset);
        }
        for offset in offset_vec {
            let entry = self.read_with_offset(offset.clone())?;
            data.push(entry);
        }
        return Ok(data);
    }

    pub fn add(&mut self, key: String, value: Value) -> Result<()> {
        let value_size: usize = bincode::serialize(&value)?.len();
        let entry = Entry::add(key.clone(), value, value_size);
        let size = self.write(&entry)? as u64;
        self.file_writer.flush()?;
        if let Some(pos) = self.index.get(&key) {
            let last_invalid_entry = self.read_with_offset(*pos)?;
            self.uncompacted += last_invalid_entry.size() as u64;
        }
        self.index.insert(key, self.position - size);
        Ok(())
    }
    
    pub fn delete(&mut self, key: String) -> Result<()> {
        if let Some(pos) = self.index.get(&key) {
            let invalid_add_entry = self.read_with_offset(*pos)?;
            self.index.remove(&key);
            let entry = Entry::delete(key);
            let size = self.write(&entry)?;
            self.file_writer.flush()?;
            self.uncompacted += size;
            self.uncompacted += invalid_add_entry.size() as u64;

            return Ok(());
        }
        Err(KvError::KeyNotFound(key))
    }

    pub fn compact(&mut self) -> Result<()> {
        let new_filename = self.path.clone() + ".compact";
        let mut new_file_writer = BufWriter::new(OpenOptions::new().write(true).create(true).open(new_filename.clone())?);
        let mut new_position = 0;
        let mut offset = 0;
        let mut new_hashmap: HashMap<String, u64> = HashMap::new();
        loop {
            match self.read_with_offset(offset) {
                Ok(entry) => {
                    let size = entry.size() as u64;
                    if let Some(pos) = self.index.get(&entry.key) {
                        if entry.meta.command == Command::Add && *pos == offset {
                            new_hashmap.insert(entry.key.clone(),new_position);
                            let buf = entry.encode()?; 
                            new_file_writer.write(&buf)?;
                            new_position += size;
                        }
                    }
                    offset += size;
                },
                Err(KvError::EOF) => break,
                Err(e) => return Err(e),
            }
        }
        new_file_writer.flush()?;
        fs::rename(&new_filename, &self.path)?;
        self.file_writer = new_file_writer;
        self.file_reader = BufReader::new(File::open(&self.path)?);
        self.position = new_position;
        self.uncompacted = 0;
        self.index = new_hashmap;
        Ok(())
    }

    pub fn type_of(value: Value) -> String {
        return match value {
            Value::Null => "Null".to_string(),
            Value::Bool(_) => "Bool".to_string(),
            Value::Int32(_) => "Int".to_string(),
            Value::Int64(_) => "Long".to_string(),
            Value::Float32(_) => "Float".to_string(),
            Value::Float64(_) => "Double".to_string(),
            Value::String(_) => "String".to_string(),
            Value::Char(_) => "Char".to_string(),
            Value::Array(_) => "Array".to_string(),
        }
    }

    fn load_hashmap(&mut self) -> Result<(HashMap<String, u64>, u64)> {
        let mut offset = 0;
        let mut new_hashmap: HashMap<String, u64> = HashMap::new();
        let mut uncompacted: u64 = 0;
        loop {
            match self.read_with_offset(offset) {
                Ok(entry) => {
                    let size = entry.size() as u64;
                    if let Some(pos) = new_hashmap.get(&entry.key) {
                        let last_invalid_entry = self.read_with_offset(*pos)?;
                        uncompacted += last_invalid_entry.size() as u64;
                    }
                    match entry.meta.command {
                        Command::Add => {new_hashmap.insert((*entry.key).to_string(), offset);}
                        Command::Delete => {
                            uncompacted += size;
                            new_hashmap.remove(&entry.key);
                        }
                    }
                    offset += size;
                },
                Err(KvError::EOF) => {break;}
                Err(e) => return Err(e),
            }
        }
        Ok((new_hashmap,uncompacted))
    }

    fn write(&mut self, entry: &Entry) -> Result<u64> {
        if self.uncompacted >= COMPACTION_THRESHOLD {
            self.compact()?;
        }
        let buf = entry.encode()?; 
        let size = buf.len() as u64;
        self.position += size;
        self.file_writer.write(&buf)?;
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
                let value: Value = bincode::deserialize(&value_buf.as_mut_slice())?;
                let result: Entry = match entry_meta.command {
                    Command::Add => {
                        Entry {
                            meta: Meta {
                                command: Command::Add,
                                key_size: entry_meta.key_size,
                                value_size: entry_meta.value_size,
                            },
                            key: key,
                            value: value,
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
                            value: Value::Null,
                        }
                    }
                };
                Ok(result)
            },
            Err(e) => Err(e),
        };
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::String(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
            Value::Int32(v) => write!(f, "{}", v),
            Value::Int64(v) => write!(f, "{}", v),
            Value::Float32(v) => write!(f, "{}", v),
            Value::Float64(v) => write!(f, "{}", v),
            Value::Char(v) => write!(f, "{}", v),
            Value::Array(v) => write!(f, "{:?}", v),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.key, self.value)
    }
}