use std::{
    io::{
        Read, 
        BufWriter,
    },
    net::{SocketAddr, TcpListener, TcpStream, Shutdown},
    fs::File,
    thread,
    time,
};
use toml;
use super::error::{KvError,Result};
use super::kv::{DataStore,Value};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    name: String,
    ip: String,
    port: String,
}

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
        let config = match get_config() {
            Ok(config) => config,
            Err(_e) => Config {
                name: "Default server".to_string(),
                ip: "127.0.0.1".to_string(),
                port: "11451".to_string(),
            },
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
fn get_config() -> Result<Config> {
    let mut file = File::open("config/server.toml")?;
    let mut c = String::new();
    file.read_to_string(&mut c)?;
    let config: Config = toml::from_str(c.as_str())?;
    Ok(config)
}