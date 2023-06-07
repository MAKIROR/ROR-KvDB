use std::io::{self, Write};
use super::{
    error::{RorError, Result},
    store::{
        kv::{DataStore, Value},
    },
    client::Client,
    request::*,
    user::user::User,
    cmd::{
        parser::Parser,
        statement::*,
    }
};
use chrono::prelude::Local;

pub struct LocalRepl {
    database: DataStore,
}

impl LocalRepl {
    pub fn open(path: &str) -> Result<Self> {
        let database = DataStore::open(path)?;
        Ok(Self {
            database,
        })
    }

    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.match_command() {
                println!("{}",e);
            }
        }
    }

    pub fn match_command(&mut self) -> Result<()> {
        print!("{0} > ", self.database.path);
        io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input == "\n" {
            return Ok(())
        }
        let mut parser = Parser::new();
        match parser.parse(&input)? {
            Statement::Open { file } => {
                self.database = DataStore::open(&file)?;
                println!("successfully opened '{}' \n", file);
            },
            Statement::Add { key, value, datatype } => {
                let db_value = to_value(value.clone(), datatype)?;
                self.database.add(key.clone(), db_value.clone())?;
                println!("Successfully added data {0} : {1}\n", key, value);
            },
            Statement::Delete { key } => {
                self.database.delete(key.clone())?;
                println!("Successfully delete data {}\n", key);
            },
            Statement::Get { key } => {
                let value = self.database.get(key)?;
                println!("{}\n", value);
            },
            Statement::Compact => {
                self.database.compact()?;
                println!("Datafile {} has been compacted\n", self.database.path);
            },
            Statement::TypeOf { key } => {
                let value = self.database.get(key)?;
                println!("{}\n", DataStore::type_of(value));
            },
            Statement::List { list } => {
                match list {
                    List::Values => {
                        let data = self.database.get_all_value()?;
                        let mut s = String::new();
                        for value in data {
                            s = format!("{}\n{}", s, value);
                        }
                        println!("{}\n", s);
                    },
                    List::Entries => {
                        let data = self.database.get_all_entry()?;
                        let mut s = String::new();
                        for entry in data {
                            s = format!("{}\n{}", s, entry);
                        }
                        println!("{}\n", s);
                    }
                }
            },
            Statement::User { cmd } => {
                match cmd {
                    UserCmd::Create { info } => {
                        User::register(info.name.clone(), info.password, info.level)?;
                        println!("Successfully create user '{0}\n'", info.name);   
                    },
                    UserCmd::Delete { name } => {
                        User::delete(name.clone())?;
                        println!("Successfully delete user '{0}\n'", name); 
                    }
                }
            },
            Statement::Quit => quit_program()
        }
        Ok(())
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
            match self.match_command() {
                Ok(()) => continue,
                Err(RorError::ConnectionLost(op)) => {
                    output_prompt("Lost connection, trying to reconnect...");
                    match self.reconnect() {
                        Ok(()) => {
                            let result = match self.client.operate(op) {
                                Ok(r) => r,
                                Err(e) => {
                                    program_crash(e);
                                    break;
                                }
                            };
                            Self::match_op_reply(result);
                            continue;
                        },
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
    
    fn match_command(&mut self) -> Result<()> {
        print!("{0}:{1} > ",self.info.ip, self.info.port);
        io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input == "\n" {
            return Ok(())
        }
        let mut parser = Parser::new();
        let op = match parser.parse(&input)? {
            Statement::Add { key, value, datatype } => {
                OperateRequest::Add {
                    key,
                    value: to_value(value, datatype)?,
                }
            },
            Statement::Delete { key } => OperateRequest::Delete { key },
            Statement::Get { key } => OperateRequest::Get { key },
            Statement::Compact => OperateRequest::Compact,
            Statement::TypeOf { key } => OperateRequest::GetType { key },
            Statement::Open { file: _ } => return Ok(()),
            Statement::List { list: _ } => return Ok(()),
            Statement::User { cmd } => {
                match cmd {
                    UserCmd::Create { info } => {
                        OperateRequest::CreateUser {
                            name: info.name,
                            password: info.password,
                            level: info.level,
                        }  
                    },
                    UserCmd::Delete { name } => {
                        OperateRequest::DeleteUser { name }
                    }
                }
            },
            Statement::Quit => {
                let _ = self.client.operate(OperateRequest::Quit);
                quit_program();
                return Ok(())
            }
        };
        let result = self.client.operate(op)?;
        Self::match_op_reply(result);
        Ok(())
    }

    fn match_op_reply(result: OperateResult) {
        match result {
            OperateResult::Found(_) => (),
            OperateResult::Type(t) => println!("{}\n", t),
            OperateResult::Success => println!("Successfully completed the request\n"),
            OperateResult::PermissionDenied => println!("Permission Denied\n"),
            OperateResult::KeyNotFound => println!("Key not found\n"),
            OperateResult::Failure => println!("The request failed, possibly due to a server error\n"),
        }
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

fn to_value(v: ValueP, data_type: ValueType) -> Result<Value> {
    let value = match data_type {
        ValueType::Null => Value::Null,
        ValueType::Bool => {
            if let ValueP::Bool(b) = v {
                Value::Bool(b);
            }
            return Err(RorError::ConvertError(v.get_str(), data_type));
        },
        ValueType::Int32 => {
            match v.get_str().parse::<i32>() {
                Ok(c) => Value::Int32(c),
                Err(_) => return Err(RorError::ConvertError(v.get_str(), data_type)),
            }
        }
        ValueType::Int64 => {
            match v.get_str().parse::<i64>() {
                Ok(c) => Value::Int64(c),
                Err(_) => return Err(RorError::ConvertError(v.get_str(), data_type)),
            }
        }
        ValueType::Float32 => {
            match v.get_str().parse::<f32>() {
                Ok(c) => Value::Float32(c),
                Err(_) => return Err(RorError::ConvertError(v.get_str(), data_type)),
            }
        }
        ValueType::Float64 => {
            match v.get_str().parse::<f64>() {
                Ok(c) => Value::Float64(c),
                Err(_) => return Err(RorError::ConvertError(v.get_str(), data_type)),
            }
        }
        ValueType::Char => {
            match v.get_str().parse::<char>() {
                Ok(c) => Value::Char(c),
                Err(_) => return Err(RorError::ConvertError(v.get_str(), data_type)),
            }
        },
        ValueType::String => Value::String(v.get_str()),
        ValueType::Array(ref include_type) => {
            if let ValueP::Array(array) = v.clone() {
                let mut values = Vec::new();
                for i in array.iter() {
                    let db_value = to_value(i.clone(), *(include_type.clone()))?;
                    values.push(db_value);
                }
                Value::Array(Box::new(values))
            } else {
                return Err(RorError::ConvertError(v.get_str(), data_type));
            }
        }
    };

    Ok(value)
}

fn output_prompt(content: &str) {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("\n[{0}] {1}",time,content);
}

fn quit_program() {
    println!("Quit ROR Database");
    std::process::exit(0);
}

fn program_crash(e: RorError) {
    println!("Program exits: {}",e);
    println!("Quit ROR Database");
    std::process::exit(0);
}