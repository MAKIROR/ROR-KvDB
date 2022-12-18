use std::fs;
use std::io::{Error, Write};

pub struct DataStore {
    reader: BufReader<File>,
    writer: BufWriter<File>,
    index: HashMap<String, CommandPos>,
}

struct CommandPos {
    pos: u64,
    len: u64,
}

impl DataStore {
    pub fn Open( path : String ) -> Result<Self, Error>  {
        
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
    pub fn Set(&mut self, key: String, value: String) -> Result<()> {
    }
}