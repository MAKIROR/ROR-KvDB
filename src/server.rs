use std::{
    io::{
        Read,
        Write,
    },
    net::{TcpListener, TcpStream, Shutdown, SocketAddr},
    fs::File,
    collections::HashMap,
    sync::{Arc,Mutex},
    thread,
    time,
    path::PathBuf,
};
use super::{
    error::{RorError,Result},
    store::kv::{DataStore,Value,USIZE_SIZE},
    user::{
        user::User,
        user_error::UserError,
    },
    request::*,
};
use serde::{Serialize,Deserialize};
use same_file::is_same_file;
use chrono::prelude::Local;
use bincode;

pub struct Client {
    stream: TcpStream,
    db: Arc<Mutex<DataStore>>,
    user: User,
}

pub struct Server {
    config: Config,
    dbs: HashMap<String,Arc<Mutex<DataStore>>>,
}

impl Server {
    pub fn new() -> Self {
        let config: Config = match Config::get_server() {
            Ok(config) => config,
            Err(_e) => Config::default(),
        };
        return Self {
            config,
            dbs: HashMap::new(),
        };
    }
    pub fn start(&mut self) -> Result<()> {
        let address = self.config.ip.clone() + ":" + &self.config.port.clone();
        let listener = TcpListener::bind(address.clone())?;
        output_prompt(format!("Server start: {}", address).as_str());
        'outer: loop {
            let (mut stream, adr) = listener.accept()?;
            output_prompt(format!("New connection: {}", adr).as_str());

            match self.handle_connection(stream,adr) {
                Ok(()) => continue,
                Err(e) => output_prompt(
                    format!("Client [{0}], failed to login. reason: {1}", 
                        adr, 
                        e,
                    ).as_str()
                ),
            }
        }
        Ok(())
    }
    fn handle_connection(&mut self, mut stream: TcpStream, address: SocketAddr) -> Result<()> {
        let mut size_buffer = [0 as u8; USIZE_SIZE];
        stream.read_exact(&mut size_buffer)?;
        let size = usize::from_be_bytes(size_buffer);
        let mut head_buffer = vec![0; size];
        stream.read_exact(&mut head_buffer)?;

        let head: ConnectRequest = match bincode::deserialize(&head_buffer) {
            Ok(buf) => buf,
            Err(e) => {
                Self::send_connect_error(stream, ConnectError::RequestError)?;
                return Err(RorError::BincodeError(e));
            },
        };
        let user = match User::login(head.user_name,head.password) {
            Ok(u) => u,
            Err(UserError::UserNotFound(n)) => {
                Self::send_connect_error(stream, ConnectError::UserNotFound(n.clone()))?;
                return Err(RorError::UserError(UserError::UserNotFound(n)));
            },
            Err(UserError::WrongPassWord) => {
                Self::send_connect_error(stream, ConnectError::PasswordError)?;
                return Err(RorError::UserError(UserError::WrongPassWord));
            },
            Err(UserError::SerdeJsonError(e)) => {
                Self::send_connect_error(stream, ConnectError::ServerError)?;
                return Err(RorError::UserError(UserError::SerdeJsonError(e)));
            }
            Err(e) => {
                Self::send_connect_error(stream, ConnectError::ServerError)?;
                return Err(RorError::UserError(e));
            }
        };

        let mut db_path_buf = PathBuf::new();
        db_path_buf.push(&self.config.data_path);
        db_path_buf.push(&head.db_path);
        let mut db_path = match db_path_buf.into_os_string().into_string() {
            Ok(s) => s,
            Err(e) => {
                Self::send_connect_error(stream, ConnectError::PathError)?;
                return Err(RorError::PathError);
            },
        };

        let opened_db;
        for (key, db) in &self.dbs {
            let exists = match is_same_file(&key, &db_path) {
                Ok(b) => b,
                Err(e) => {
                    Self::send_connect_error(stream, ConnectError::ServerError)?;
                    return Err(RorError::IOError(e));
                },
            };
            if exists {
                db_path = key.clone();
                opened_db = Arc::clone(db);
                if let Err(_) = Self::send_connect_reply(&mut stream, &user){
                    stream.shutdown(Shutdown::Both)?;
                }
                let client = Client {
                    stream,
                    db: opened_db,
                    user,
                };
                thread::spawn(|| {
                    Self::handle_client(client);
                });
                return Ok(())
            }
        }
        opened_db = match self.open_new_db(db_path.clone()) {
            Ok(db) => db,
            Err(e) => {
                Self::send_connect_error(stream, ConnectError::OpenFileError)?;
                return Err(e);
            },
        };
        if let Err(_) = Self::send_connect_reply(&mut stream, &user) {
            stream.shutdown(Shutdown::Both)?;
        }
        let client = Client {
            stream,
            db: opened_db,
            user,
        };
        thread::spawn(|| {
            Self::handle_client(client);
        });
        Ok(())
    }


    fn send_connect_error(mut stream: TcpStream, err: ConnectError) -> Result<()> {
        let err_bytes = bincode::serialize(&ConnectReply::Error(err))?;
        let size = err_bytes.len();
        let mut buf = vec![0; USIZE_SIZE + size];
        buf[0..USIZE_SIZE].copy_from_slice(&size.to_be_bytes());
        buf[USIZE_SIZE..].copy_from_slice(&err_bytes);
        stream.write(buf.as_slice())?;
        stream.shutdown(Shutdown::Both)?;
        Ok(())
    }
    fn send_connect_reply(stream: &mut TcpStream, user: &User) -> Result<()> {
        let msg_bytes = bincode::serialize(&ConnectReply::Success(user.clone()))?;
        let size = msg_bytes.len();
        let mut buf = vec![0; USIZE_SIZE + size];
        buf[0..USIZE_SIZE].copy_from_slice(&size.to_be_bytes());
        buf[USIZE_SIZE..].copy_from_slice(&msg_bytes);
        stream.write(buf.as_slice())?;
        Ok(())
    }
    fn handle_client(mut client: Client) {
        loop {
            let mut cmd_buffer: Vec<u8> = Vec::new();
            let read_result = client.stream.read(&mut cmd_buffer);
            match read_result {
                Ok(buf_len) => {
                    if buf_len == 0 {
                        continue;
                    }
                    let command: OperateRequest = match bincode::deserialize(&cmd_buffer) {
                        Ok(cmd) => cmd,
                        Err(e) => {
                            let err = bincode::serialize(&OperateResult::Failure).unwrap();
                            client.stream.write(err.as_slice()).unwrap();
                            continue;
                        }
                    };
                    match Self::match_command(&mut client,command) {
                        Ok(()) => continue,
                        Err(RorError::Disconnect) => break,
                        Err(e) => {
                            let err = bincode::serialize(&OperateResult::Failure).unwrap();
                            client.stream.write(err.as_slice()).unwrap();
                            continue;
                        }
                    }
                },
                Err(_) => {
                    client.stream.shutdown(Shutdown::Both).unwrap();
                    break;
                }
            }
        }
    }
    fn open_new_db(&mut self, path: String) -> Result<Arc<Mutex<DataStore>>> {
        let db = Arc::new(
            Mutex::new(
                DataStore::open(path.as_str())?
            )
        );
        let arc_clone_db = Arc::clone(&db);
        self.dbs.insert( path.clone(), db );
        Ok(arc_clone_db)
    }
    fn match_command(client: &mut Client, command: OperateRequest) -> Result<()> {
        match command {
            OperateRequest::Get { key } => {
                match client.db.lock().unwrap().get(&key) {
                    Ok(v) => {
                        let value = bincode::serialize(&OperateResult::Success(v))?;
                        client.stream.write(value.as_slice())?;
                        return Ok(());
                    }
                    Err(e) => return Err(RorError::KvError(e)),
                }
            }
            OperateRequest::Delete { key } => {
                if client.user.level != "3" && client.user.level != "4" {
                    let result = bincode::serialize(&OperateResult::PermissionDenied)?;
                    client.stream.write(result .as_slice())?;
                    return Ok(());
                }
                match client.db.lock().unwrap().delete(&key) {
                    Ok(_s) => {
                        let value = bincode::serialize(&OperateResult::Success(Value::Null))?;
                        client.stream.write(value.as_slice())?;
                        return Ok(());
                    }
                    Err(e) => return Err(RorError::KvError(e)),
                }
            }
            OperateRequest::Add { key, value } => {
                if client.user.level != "2" && client.user.level != "3" && client.user.level != "4" {
                    let result = bincode::serialize(&OperateResult::PermissionDenied)?;
                    client.stream.write(result.as_slice())?;
                    return Ok(());
                }
                match client.db.lock().unwrap().add(&key,value) {
                    Ok(_) => {
                        let value = bincode::serialize(&OperateResult::Success(Value::Null))?;
                        client.stream.write(value.as_slice())?;
                        return Ok(());
                    }
                    Err(e) => return Err(RorError::KvError(e)),
                }
            }
            OperateRequest::Quit => {
                client.stream.shutdown(Shutdown::Both)?;
                return Err(RorError::Disconnect);
            },
        }
    }
}

#[derive(Deserialize,Serialize)]
struct Config {
    name: String,
    ip: String,
    port: String,
    worker_id: i64,
    data_center_id: i64,
    data_path: String,
}

impl Config {
    fn default() -> Self {
        Config {
            name: "Default server".to_string(),
            ip: "127.0.0.1".to_string(),
            port: "11451".to_string(),
            worker_id: 0,
            data_center_id: 0,
            data_path: "./data/".to_string(),
        }
    }
    pub fn get_server() -> Result<Self> {
        let mut file = File::open("config/server.toml")?;
        let mut c = String::new();
        file.read_to_string(&mut c)?;
        let config: Config  = toml::from_str(c.as_str())?;
        Ok(config)
    }
    pub fn set_server(&self) -> Result<()> {
        let mut file = File::create("config/server.toml")?;
        let toml = toml::to_string(&self)?;
        write!(file, "{}", toml)?;
        Ok(())
    }
}

fn output_prompt<T: std::fmt::Display>(content: T) {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("[{0}] {1}",time,content);
}