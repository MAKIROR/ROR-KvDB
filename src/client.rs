use std::net::TcpStream;
use super::{
    error::{RorError,Result},
    store::kv::{DataStore,Value},
    user::user::User,
    request::*,
};

pub strcut Client {
    stream: TcpStream,
    user: User,
}

impl Client {
    pub fn connect(ip: &str, port: &str, user_name: &str, password: &str) -> Result<Self> {
        let address = ip + port;
        let stream = match TcpStream::connect(address) {
            OK(s) => s,
            Err(_) => todo!(),
        }
    }
}