use std::io::{self, Write};
use super::error::{KvError,Result};
use super::kv::{DataStore,Value};

pub struct RorDb {
    pub database: DataStore,
}

impl RorDb {
    pub fn open(path: String) -> Result<DataStore> {
        let db = DataStore::open(path)?;
        Ok(db)
    }
    pub fn run(db: DataStore) {
        let mut rordb = RorDb {database:db};
        loop {
            if let Err(e) = rordb.match_command() {
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
                if command.len() != 2 {
                    return Err(KvError::ParameterError("open".to_string()));
                }
                let db = DataStore::open((*command[1]).to_string())?;
                self.database = db;
                println!("successfully opened '{}' ", command[1]);
            }
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
                        "i32" | "int" => {
                            match command[2].parse::<i32>() {
                                Ok(value) => Value::Int32(value),
                                Err(_e) => return Err(KvError::ConvertError(command[2].to_string(), "Int32".to_string())),
                            }
                        }
                        "i64" | "long" => {
                            match command[2].parse::<i64>() {
                                Ok(value) => Value::Int64(value),
                                Err(_e) => return Err(KvError::ConvertError(command[2].to_string(), "Int64".to_string())),
                            }
                        }
                        "f32" | "float" => {
                            match command[2].parse::<f32>() {
                                Ok(value) => Value::Float32(value),
                                Err(_e) => return Err(KvError::ConvertError(command[2].to_string(), "Float32".to_string())),
                            }
                        }
                        "f64" | "double" => {
                            match command[2].parse::<f64>() {
                                Ok(value) => Value::Float64(value),
                                Err(_e) => return Err(KvError::ConvertError(command[2].to_string(), "Float64".to_string())),
                            }
                        }
                        "char" => Value::Char(command[2].chars().collect()),
                        "string" => Value::String(command[2].to_string()),
                        _ => return Err(KvError::UnknownType(command[3].to_string())),
                    };
                    self.database.add(command[1].to_string(),value)?;
                } else if command.len() == 3 {
                    self.database.add(command[1].to_string(), Value::String(command[2].to_string()))?;
                } else {
                    return Err(KvError::ParameterError("add".to_string()));
                }
                println!("Successfully added data {0} : {1}",command[1],command[2]);
            }
            "delete" => {
                if command.len() != 2 {
                    return Err(KvError::ParameterError("delete".to_string()));
                }
                self.database.delete(command[1].to_string())?;
                println!("Successfully delete data {}",command[1]);
            }, 
            "compact" => {
                if command.len() != 1 {
                    return Err(KvError::ParameterError("compact".to_string()));
                }
                self.database.compact()?;
                println!("Datafile {} has been compacted", self.database.path);
            },
            "get" => {
                if command.len() != 2 {
                    return Err(KvError::ParameterError("get".to_string()));
                }
                let value: Value = self.database.get(command[1].to_string())?;
                let str_value = match value {
                    Value::Null => "Null".to_string(),
                    Value::String(v) => v,
                    Value::Bool(v) => v.to_string(),
                    Value::Int32(v) => v.to_string(),
                    Value::Int64(v) => v.to_string(),
                    Value::Float32(v) => v.to_string(),
                    Value::Float64(v) => v.to_string(),
                    Value::Char(v) => {
                        let mut string = String::new();
                        for c in v {
                            string.push(c);
                        }
                        string
                    }
                };
                println!("{}",str_value);
            }
            "quit" => quit_program(),
            _ => return Err(KvError::UnknownCommand(command[0].to_string())),
        }
        Ok(())
    }
}

pub fn quit_program() {
    println!("Quit ROR Database");
    std::process::exit(0);
}