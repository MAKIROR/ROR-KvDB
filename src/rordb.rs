use std::{
    io::{self, Write},
};
use super::error::{KvError,Result};
use super::kv::{DataStore,Value};

pub struct RorDb {
    pub database: DataStore,
}

impl RorDb {
    pub fn run(mut self) {
        loop {
            if let Err(e) = self.match_command() {
                println!("{}",e)
            }
        }
    }
    pub fn match_command(&mut self) -> Result<()> {
        print!("{0} > ",self.database.path);
        io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input == "\n" {
            return Ok(())
        }
        input = input.trim().to_string();
        let command :Vec<&str> = input.split(" ").collect();  
        match command[0] {
            "open" => {
                let db = DataStore::open((*command[1]).to_string())?;
                self.database = db;
                println!("successfully opened '{}' ", command[1]);
            }
            //self.database.add((*command[1]).to_string(),(*command[2]).to_string())?
            "add" => {
                if command.len() == 4 {
                    let value: Value = match command[3] {
                        "null" => Value::Null,
                        "bool" => {
                            match command[2] {
                                "true" | "True"| "TRUE" => Value::Bool(true),
                                "false" | "False" | "FALSE" => Value::Bool(false),
                                _ => return Err(KvError::ConvertError(command[2].to_string(), "Bool".to_string())),
                            }
                        }
                        "string" => Value::String(command[2].to_string()),
                        _ => return Err(KvError::UnknownType(command[3].to_string())),
                    };
                    self.database.add(command[1].to_string(),value)?;
                }
                self.database.add(command[1].to_string(), Value::String(command[2].to_string()))?;
            }
            "delete" => self.database.delete(command[1].to_string())?, 
            "compact" => self.database.compact()?,
            _ => println!("Unknown command '{}'", command[0]),
        }
        Ok(())
    }
}