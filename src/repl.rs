use std::io::{self, Write};
use super::{
    error::{RorError,Result},
    store::{
        kv::{DataStore,Value},
        kv_error::KvError,
    },
    client::Client,
    request::OperateRequest,
    user::user::User,
};
use chrono::prelude::Local;

pub struct LocalRepl {
    pub database: DataStore,
}

impl LocalRepl {
    pub fn open(path: &str) -> Result<Self> {
        let database = DataStore::open(path)?;
        Ok(Self{database})
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
                    return Err(RorError::ParameterError("open".to_string()));
                }
                let db = DataStore::open(command[1])?;
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
                                _ => return Err(RorError::ConvertError(command[2].to_string(), "Bool".to_string())),
                            }
                        }
                        "i32" | "int" => {
                            match command[2].parse::<i32>() {
                                Ok(value) => Value::Int32(value),
                                Err(_) => return Err(RorError::ConvertError(command[2].to_string(), "Int32".to_string())),
                            }
                        }
                        "i64" | "long" => {
                            match command[2].parse::<i64>() {
                                Ok(value) => Value::Int64(value),
                                Err(_) => return Err(RorError::ConvertError(command[2].to_string(), "Int64".to_string())),
                            }
                        }
                        "f32" | "float" => {
                            match command[2].parse::<f32>() {
                                Ok(value) => Value::Float32(value),
                                Err(_) => return Err(RorError::ConvertError(command[2].to_string(), "Float32".to_string())),
                            }
                        }
                        "f64" | "double" => {
                            match command[2].parse::<f64>() {
                                Ok(value) => Value::Float64(value),
                                Err(_) => return Err(RorError::ConvertError(command[2].to_string(), "Float64".to_string())),
                            }
                        }
                        "char" => Value::Char(command[2].chars().collect()),
                        "string" => Value::String(command[2].to_string()),
                        _ => return Err(RorError::UnknownType(command[3].to_string())),
                    };
                    self.database.add(command[1],value)?;
                } else if command.len() == 3 {
                    self.database.add(command[1], Value::String(command[2].to_string()))?;
                } else {
                    return Err(RorError::ParameterError("add".to_string()));
                }
                println!("Successfully added data {0} : {1}",command[1],command[2]);
            }
            "delete" => {
                if command.len() != 2 {
                    return Err(RorError::ParameterError("delete".to_string()));
                }
                self.database.delete(command[1])?;
                println!("Successfully delete data {}",command[1]);
            }, 
            "compact" => {
                if command.len() != 1 {
                    return Err(RorError::ParameterError("compact".to_string()));
                }
                self.database.compact()?;
                println!("Datafile {} has been compacted", self.database.path);
            },
            "get" => {
                if command.len() != 2 {
                    return Err(RorError::ParameterError("get".to_string()));
                }
                let value: Value = self.database.get(command[1])?;
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
            "user" => {
                if command[1] == "create" {
                    if command.len() != 5 {
                        return Err(RorError::ParameterError("create user".to_string()));
                    }
                    let mut user = User::new(0,0,command[2],command[3],command[4])?;
                    user.register()?;
                    println!("Successfully create user '{0}'",command[2]);   
                }
            }
            "quit" => Self::quit_program(),
            _ => return Err(RorError::UnknownCommand(command[0].to_string())),
        }
        Ok(())
    }
    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.match_command() {
                println!("{}",e);
            }
        }
    }
    fn quit_program() {
        println!("Quit ROR Database");
        std::process::exit(0);
    }
}

#[derive(Clone)]
pub struct ConnectionInfo {
    ip: String,
    port: String,
    user_name: String, 
    password: String, 
    db_path: String,
}

pub struct RemoteRepl {
    pub client: Client,
    info: ConnectionInfo,
}

impl RemoteRepl {
    pub fn new(
        ip: String,
        port: String,
        user_name: String, 
        password: String, 
        db_path: String
    ) -> Result<Self> {
        let client = Client::connect(
            ip.clone(),
            port.clone(),
            user_name.clone(),
            password.clone(),
            db_path.clone()
        )?;
        let info = ConnectionInfo {
            ip: ip,
            port: port,
            user_name: user_name,
            password: password,
            db_path: db_path,
        };
        Ok(Self {client,info})
    }
    pub fn run(&mut self) {
        loop {
            match self.process_command() {
                Ok(()) => continue,
                Err(RorError::ConnectionLost) => {
                    output_prompt("Lost connection, trying to reconnect...");
                    match self.reconnect() {
                        Ok(()) => continue,
                        Err(_) => {
                            output_prompt("Connection lost, unable to reconnect, quit program");
                            break;
                        }
                    }
                }
                Err(e) => println!("{}",e),
            }
        }
    }
    fn process_command(&mut self) -> Result<()> {
        print!("{0}:{1} > ",self.info.ip, self.info.port);
        io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input == "\n" {
            return Ok(())
        }
        input = input.trim().to_string();
        let command :Vec<&str> = input.split(" ").collect();  
        Ok(())
    }
    fn reconnect(&mut self) -> Result<()> {
        let info = self.info.clone();
        self.client = Client::connect(
            info.ip,
            info.port,
            info.user_name,
            info.password,
            info.db_path
        )?;
        Ok(())
    }
}

fn output_prompt(content: &str) {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("\n[{0}] {1}",time,content);
}