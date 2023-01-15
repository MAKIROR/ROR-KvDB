use std::{
    io::{
        Read,
        Write,
    },
    net::{SocketAddr, TcpListener, TcpStream, Shutdown},
    fs::{File,OpenOptions},
    thread,
    time,
};
use super::error::{KvError,Result};
use super::kv::{DataStore,Value};
use serde::{Serialize,Deserialize};

pub struct Client {
    address: SocketAddr,
    stream: TcpStream,
    database: DataStore,
}

pub struct Server {
    config: Config,
    clients: Vec<thread::JoinHandle<()>>,
}

impl Server {
    pub fn new() -> Self {
        let config = match Config::get_server() {
            Ok(config) => config,
            Err(_e) => Config::default,
        };
        return Self {
            config,
            clients: Vec::new()
        };
    }
    pub fn start(&mut self) -> Result<()> {
        let address = self.config.ip.clone() + ":" + &self.config.port.clone();
        let listener = TcpListener::bind(address)?;
        loop {
            let (stream, client_address) = listener.accept()?;
            let thread = thread::spawn(move|| {
                handle_client(client_address, stream);
            });
            self.clients.push(thread);
        }
        Ok(())
    }
}

fn handle_client(address: SocketAddr, mut stream: TcpStream) -> Result<()> {
    let mut head = [0 as u8; 1024];
    stream.read(&mut head)?;
    //todo
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
    Ok(())
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
