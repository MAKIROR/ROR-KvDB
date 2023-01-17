use std::{
    io::{
        Read,
        Write,
    },
    net::{SocketAddr, TcpListener, TcpStream, Shutdown},
    fs::{File,OpenOptions},
    sync::Mutex,
    thread,
    time,
};
use super::{
    error::{RorError,Result},
    store::kv::{DataStore,Value},
    user::user::User,
    request::{ConnectRequest,OperateRequest},
};
use serde::{Serialize,Deserialize};
use same_file::is_same_file;
use bincode;

pub struct Client {
    address: SocketAddr,
    stream: TcpStream,
    db_path: String,
    user: User,
}

pub struct Server {
    config: Config,
    clients: Vec<Client>,
    dbs: Vec<Mutex<DataStore>>
}

impl Server {
    pub fn new() -> Self {
        let config: Config = match Config::get_server() {
            Ok(config) => config,
            Err(_e) => Config::default(),
        };
        return Self {
            config,
            clients: Vec::new(),
            dbs: Vec::new(),
        };
    }
    pub fn start(&mut self) -> Result<()> {
        let address = self.config.ip.clone() + ":" + &self.config.port.clone();
        let listener = TcpListener::bind(address)?;
        loop {
            let (mut stream, client_address) = listener.accept()?;
            let thread = thread::spawn(move|| {
                Self::handle_client(stream, client_address);
            });
            //todo
        }
        Ok(())
    }
    fn handle_client(mut stream: TcpStream, address: SocketAddr) -> Result<()> {
        let mut head_buffer: Vec<u8> = Vec::new();
        stream.read(&mut head_buffer)?;
        let head: ConnectRequest = bincode::deserialize(&head_buffer)?;
        /*
        let mut data = [0 as u8; 50];
        loop {
            match stream.read(&mut data) {
                Ok(size) => {
                    todo!()
                },
                Err(_) => {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
            }
        }
        */
        stream.shutdown(Shutdown::Both)?;
        Ok(())
    }
}

impl Client {
    pub fn new(address: SocketAddr, stream: TcpStream, db_path: String, user: User) -> Self {
        Client {
            address,
            stream,
            db_path,
            user,
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
}

impl Config {
    fn default() -> Self {
        Config {
            name: "Default server".to_string(),
            ip: "127.0.0.1".to_string(),
            port: "11451".to_string(),
            worker_id: 0,
            data_center_id: 0,
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
