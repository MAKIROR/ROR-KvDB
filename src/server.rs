use std::{
    io::{
        Read, 
        BufWriter,
    },
    net::{
        TcpListener,
        TcpStream
    },
    fs::File,
    thread,
    time,
};
use toml;
use super::error::Result;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    name: String,
    ip: String,
    port: i32,
}

pub struct Client {
    ip: String,
    stream: TcpStream,
}

pub struct Server {
    config: Config,
    clients: Vec<Client>,
}

impl Server {
    pub fn new() -> Result<Self> {
        let config: Config = Self::get_config()?;
        Ok(Self{
            config,
            clients: Vec::new()
        })
    }
    pub fn start(&mut self) -> Result<()> {
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