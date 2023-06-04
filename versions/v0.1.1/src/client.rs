use std::{
    net::TcpStream,
    io::{
        Read,
        Write,
        ErrorKind,
    },
    thread,
};
use super::{
    error::{RorError,Result},
    user::user_error::UserError,
    request::*,
};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect(
        ip: String, 
        port: String, 
        user_name: String, 
        password: String, 
        db_path: String
    ) -> Result<Self> {
        let address = format!("{0}:{1}", &ip, &port);
        let mut stream = match TcpStream::connect(&address) {
            Ok(s) => s,
            Err(e) => return Err(RorError::ConnectFailed(e)),
        };

        let (buf,_) = Message::new(ConnectRequest {
            db_path,
            user_name: user_name.clone(),
            password }
        ).as_bytes()?;
        stream.write(&buf.as_slice())?;

        let mut size_buffer = [0 as u8; USIZE_SIZE];
        stream.read_exact(&mut size_buffer)?;
        let reply_size = usize::from_be_bytes(size_buffer);
        let mut reply_buffer = vec![0; reply_size];
        stream.read_exact(&mut reply_buffer)?;

        let result: ConnectReply = bincode::deserialize(&reply_buffer)?;
        match result {
            ConnectReply::Success => return Ok(Client {stream}),
            ConnectReply::Error(ConnectError::UserNotFound) => return Err(RorError::UserError(UserError::UserNotFound(user_name))),
            ConnectReply::Error(ConnectError::PasswordError) => return Err(RorError::UserError(UserError::WrongPassWord)),
            ConnectReply::Error(ConnectError::OpenFileError) => return Err(RorError::OpenFileFailed),
            ConnectReply::Error(ConnectError::RequestError) => return Err(RorError::RequestError),
            ConnectReply::Error(ConnectError::PathError) => return Err(RorError::PathError),
            ConnectReply::Error(ConnectError::ServerError) => return Err(RorError::ServerError),
        }
    }
    
    pub fn operate(&mut self, request: OperateRequest) -> Result<OperateResult> {
        let body = Message::new(request.clone());
        let (buf,_) = body.as_bytes()?;

        if let Err(_) = self.stream.write(&buf) {
            return Err(RorError::ConnectionLost(request));
        }
        thread::sleep(std::time::Duration::from_millis(50));

        let mut size_buffer = [0 as u8; USIZE_SIZE];
        match self.stream.read_exact(&mut size_buffer) {
            Ok(_) => (),
            Err(e) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    return Err(RorError::AbnormalConnection);
                }
                return Err(RorError::IOError(e));
            }
        }
        let reply_size = usize::from_be_bytes(size_buffer);
        let mut reply_buffer = vec![0; reply_size];
        self.stream.read_exact(&mut reply_buffer)?;

        let reply: OperateResult = match bincode::deserialize(&reply_buffer) {
            Ok(r) => r,
            Err(_) => return Err(RorError::IncompleteData),
        };
        return Ok(reply);
    }
}