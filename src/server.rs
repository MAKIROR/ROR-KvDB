use std::{
    io::{
        Read,
        Write,
    },
    net::{TcpListener, TcpStream, Shutdown},
    fs::File,
    collections::HashMap,
    sync::{Arc,Mutex},
    thread,
};
use super::{
    error::{RorError,Result},
    store::kv::{DataStore,Value},
    user::{
        user::User,
    },
    request::*,
};
use serde::{Serialize,Deserialize};
use same_file::is_same_file;
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
        let listener = TcpListener::bind(address)?;
        'outer: loop {
            let (mut stream, _) = listener.accept()?;

            let mut head_buffer: Vec<u8> = Vec::new();
            stream.read(&mut head_buffer);
            let head: ConnectRequest = match bincode::deserialize(&head_buffer) {
                Ok(buf) => buf,
                Err(_) => {
                    let err_bytes = bincode::serialize(&ConnectReply::Error(ConnectError::RequestError))?;
                    stream.write(err_bytes.as_slice());
                    stream.shutdown(Shutdown::Both);
                    continue;
                },
            };
            let user = match User::login(head.user_name,head.user_password) {
                Ok(u) => u,
                Err(e) => return Err(RorError::UserError(e)),
            };
            let mut db_path = self.config.data_path.clone() + &head.db_path;
            let mut should_open = true;
            let opened_db: Arc<Mutex<DataStore>>;
            for (key, db) in &self.dbs {
                let exists = match is_same_file(&key, &db_path) {
                    Ok(b) => b,
                    Err(_) => {
                        let err_bytes = bincode::serialize(&ConnectReply::Error(ConnectError::FileError))?;
                        stream.write(err_bytes.as_slice());
                        stream.shutdown(Shutdown::Both);
                        continue 'outer;
                    },
                };
                if exists {
                    db_path = key.clone();
                    opened_db = Arc::clone(db);
                    should_open = false;
                    break;
                }
            }
            if should_open {
                let opened_db = match self.open_new_db(db_path.clone()) {
                    Ok(db) => db,
                    Err(_) => {
                        let err_bytes = bincode::serialize(&ConnectReply::Error(ConnectError::OpenFileError))?;
                        stream.write(err_bytes.as_slice());
                        stream.shutdown(Shutdown::Both);
                        continue;
                    },
                };
            }
            let client = Client {
                stream,
                db: opened_db,
                user,
            };
            thread::spawn(|| {
                Self::handle_client(client);
            });
        }
        Ok(())
    }
    fn handle_client(mut client: Client) {
        loop {
            let mut cmd_buffer: Vec<u8> = Vec::new();
            let read_result = client.stream.read(&mut cmd_buffer);
            match read_result {
                Ok(buf_len) => {
                    drop(read_result);
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
                    todo!();
                },
                Err(_) => {
                    drop(read_result);
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
    fn match_command(&mut self, client: &mut Client, command: OperateRequest) -> Result<()> {
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
                match client.db.lock().unwrap().delete(&key) {
                    Ok(v) => {
                        let value = bincode::serialize(&OperateResult::Success(Value::Null))?;
                        client.stream.write(value.as_slice())?;
                        return Ok(());
                    }
                    Err(e) => return Err(RorError::KvError(e)),
                }
            }
            OperateRequest::Add { key, value } => {
                match client.db.lock().unwrap().add(&key,value) {
                    Ok(v) => {
                        let value = bincode::serialize(&OperateResult::Success(Value::Null))?;
                        client.stream.write(value.as_slice())?;
                        return Ok(());
                    }
                    Err(e) => return Err(RorError::KvError(e)),
                }
            }
        }

    }
}

#[derive(Deserialize,Serialize)]
pub struct Config {
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
