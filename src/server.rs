use std::{
    io::{
        Read, 
        BufWriter,
    },
    net::{SocketAddr, TcpListener, TcpStream},
    fs::File,
    thread,
    time,
};
use toml;
use super::error::Result;
use serde::{Serialize,Deserialize};
use tokio;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    name: String,
    ip: String,
    port: String,
}

pub struct Client {
    ip: SocketAddr,
    stream: TcpStream,
}

pub struct Server {
    config: Config,
    clients: Vec<SocketAddr>,
}

impl Server {
    pub fn new() -> Self {
        let config = match Self::get_config() {
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
    pub async fn start(&mut self) -> Result<()> {
        let address = self.config.ip.clone() + ":" + &self.config.port.clone();
        let listener = TcpListener::bind(address).unwrap();
        loop {
            let (stream, client_address) = listener.accept()?;
            self.clients.push(client_address.clone());
            Self::handle_client(Client {
                ip: client_address,
                stream: stream, 
            }).await;
        }
        Ok(())
    }
    async fn handle_client(client: Client) {
        todo!()
    }
    fn get_config() -> Result<Config> {
        let mut file = File::open("config/server.toml")?;
        let mut c = String::new();
        file.read_to_string(&mut c)?;
        let config: Config = toml::from_str(c.as_str())?;
        Ok(config)
    }
}