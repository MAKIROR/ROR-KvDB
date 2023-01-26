use std::{
    net::TcpStream,
    io::{
        Read,
        Write,
    },
    thread,
    time,
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

pub struct Client {
    stream: TcpStream,
    user: User,
}

impl Client {
    pub fn connect(
        ip: String, 
        port: String, 
        user_name: String, 
        password: String, 
        db_path: String
    ) -> Result<Self> {
        let address = ip.clone() + ":" + &port;
        let mut stream = match TcpStream::connect(&address) {
            Ok(s) => s,
            Err(e) => {
                println!("{}",e);
                return Err(RorError::ConnectFailed(address));
            },
        };

        {
            let body = ConnectRequest::new(db_path,user_name,password);
            let (buf,_) = body.as_bytes()?;
            stream.write(&buf.as_slice())?;
        }

        let mut size_buffer = [0 as u8; USIZE_SIZE];
        stream.read_exact(&mut size_buffer)?;
        let reply_size = usize::from_be_bytes(size_buffer);
        let mut reply_buffer = vec![0; reply_size];
        stream.read_exact(&mut reply_buffer)?;

        let result: ConnectReply = bincode::deserialize(&reply_buffer)?;
        match result {
            ConnectReply::Success(user) => return Ok(Client {stream,user}),
            ConnectReply::Error(ConnectError::UserNotFound(name)) => return Err(RorError::UserError(UserError::UserNotFound(name))),
            ConnectReply::Error(ConnectError::PasswordError) => return Err(RorError::UserError(UserError::WrongPassWord)),
            ConnectReply::Error(ConnectError::OpenFileError) => return Err(RorError::OpenFileFailed),
            ConnectReply::Error(ConnectError::RequestError) => return Err(RorError::RequestError),
            ConnectReply::Error(ConnectError::PathError) => return Err(RorError::PathError),
            ConnectReply::Error(ConnectError::ServerError) => return Err(RorError::ServerError),
        }
    }
    pub fn operate(&mut self, request: OperateRequest) -> Result<OperateResult> {
        match self.stream.write(&bincode::serialize(&request)?) {
            Ok(_) => {

                let mut size_buffer = [0 as u8; USIZE_SIZE];
                self.stream.read_exact(&mut size_buffer)?;
                let reply_size = usize::from_be_bytes(size_buffer);
                let mut reply_buffer = vec![0; reply_size];
                self.stream.read_exact(&mut reply_buffer)?;

                let reply: OperateResult = match bincode::deserialize(&reply_buffer) {
                    Ok(r) => r,
                    Err(_) => return Err(RorError::IncompleteData),
                };
                return Ok(reply);
            },
            Err(_) => return Err(RorError::ConnectionLost),
        }
    }
}