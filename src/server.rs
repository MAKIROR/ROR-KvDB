use std::{
    io::{
        Read,
        Write,
        BufReader,
        ErrorKind,
    },
    fs,
    net::{TcpListener, TcpStream, Shutdown, SocketAddr},
    fs::File,
    collections::HashMap,
    sync::{Arc,Mutex},
    thread::{self, JoinHandle},
    time,
    path::{PathBuf,Path},
};
use super::{
    error::{RorError,Result},
    store::{
        kv::DataStore,
        kv_error::KvError,
    },
    user::{
        user::{self,User},
        user_error::UserError,
    },
    request::*,
    repl::RemoteRepl,
};
use serde::{Serialize,Deserialize};
use same_file::is_same_file;
use chrono::prelude::Local;
use bincode;
use colored::Colorize;

pub struct Server {
    config: Config,
    dbs: HashMap<String, Arc<Mutex<DataStore>>>,
    clients: HashMap<String, (String, JoinHandle<()>)>,
}

enum DataPath {
    Exists(Arc<Mutex<DataStore>>),
    None,
}

impl Server {
    pub fn new() -> Self {
        let config: Config = match Config::get_server() {
            Ok(config) => config,
            Err(e) => {
                output_prompt(format!("Could not read configuration file: {0}", e));
                Config::default()
            }
        };
        return Self {
            config,
            dbs: HashMap::new(),
            clients: HashMap::new(),
        };
    }
    
    pub fn init() -> Result<()> {
        fs::create_dir("config")?;
        let server_config = toml::to_string(&Config::default())?;
        let mut file = File::create("config/server.toml")?;
        write!(file, "{}", server_config)?;
        let user_config = toml::to_string(&user::Config::default())?;
        let mut file = File::create("config/user.toml")?;
        write!(file, "{}", user_config)?;
        fs::create_dir("data")?;
        File::create("data/default.data")?;

        User::test_file()?;
        User::register("root".to_string(), "123456".to_string(), "3".to_string())?;

        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        let address = format!("{0}:{1}", self.config.ip, &self.config.port);
        let listener = TcpListener::bind(address.clone())?;
        User::test_file()?;

        output_prompt(format!("Server start: {}", address));

        
        if self.config.repl {
            output_prompt(format!("Connect to local server in REPL mode, user: {}", &self.config.local_user));

            let config_copy = self.config.clone();

            thread::spawn(move || {
                let str_user = config_copy.local_user.as_str();
                let user: Vec<&str> = str_user.split("@").collect();
                if user.len() != 2 {
                    output_prompt("Failed to start REPL mode: invalid user");
                    return;
                }
                let mut repl = RemoteRepl::new(
                    config_copy.ip.clone(),
                    config_copy.port.clone(),
                    user[0].to_string(),
                    user[1].to_string(),
                    config_copy.default_db.clone()
                ).unwrap();
                repl.run();
            });
        }

        std::thread::spawn( || {

        });

        let mut accepted_times = 0;

        loop {
            if self.config.auto_refresh > 0 && accepted_times >= self.config.auto_refresh {
                output_prompt("The server starts to refresh automatically...");
                self.refresh()?;
                output_prompt("Done!");
                accepted_times = 0;
            }
            let (stream, adr) = match listener.accept() {
                Ok(r) => r,
                Err(e) => {
                    output_prompt(format!("Unable to accept connection from a client: {0}",e));
                    continue;
                }
            };
            output_prompt(format!("New connection: {}", adr));
            accepted_times += 1;

            match self.handle_connection(stream,adr) {
                Ok(()) => continue,
                Err(e) => output_prompt(
                    format!("Client [{0}], failed to login. reason: {1}", 
                        adr, 
                        e,
                    )
                ),
            }
        }
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
                Self::send_error(&mut stream, ConnectError::RequestError)?;
                return Err(RorError::BincodeError(e));
            },
        };
        let user = match User::login(head.user_name,head.password) {
            Ok(u) => u,
            Err(UserError::UserNotFound(n)) => {
                Self::send_error(&mut stream, ConnectError::UserNotFound)?;
                return Err(RorError::UserError(UserError::UserNotFound(n)));
            },
            Err(UserError::WrongPassWord) => {
                Self::send_error(&mut stream, ConnectError::PasswordError)?;
                return Err(RorError::UserError(UserError::WrongPassWord));
            },
            Err(e) => {
                Self::send_error(&mut stream, ConnectError::ServerError)?;
                return Err(RorError::UserError(e));
            }
        };
        let mut db_path_buf = PathBuf::new();
        db_path_buf.push(&self.config.data_path);
        db_path_buf.push(&head.db_path);
        let db_path = match db_path_buf.into_os_string().into_string() {
            Ok(s) => s,
            Err(_) => {
                Self::send_error(&mut stream, ConnectError::PathError)?;
                return Err(RorError::PathError);
            },
        };

        let stream_clone = match stream.try_clone() {
            Ok(s) => s,
            Err(e) => {
                output_prompt(format!("[{0}] The reader cannot be created by the clone method, and the client is disconnected", &address));
                Self::send_error(&mut stream, ConnectError::ServerError)?;
                return Err(RorError::IOError(e));
            }
        };
        let reader = BufReader::new(stream_clone);

        let target_path = Path::new(&db_path);
        if !target_path.exists() {
            File::create(&db_path)?;
        }

        let opened_db = match self.compare(db_path.clone(), &mut stream)? {
            DataPath::Exists(db) => db,
            DataPath::None => {
                let db = match self.open_new_db(db_path.clone()) {
                    Ok(db) => db,
                    Err(e) => {
                        Self::send_error(&mut stream, ConnectError::OpenFileError)?;
                        return Err(e);
                    },
                };
                db
            }
        };

        if let Err(e) = Self::send_reply(&mut stream) {
            stream.shutdown(Shutdown::Both)?;
            return Err(e);
        }
        let client = Client {
            stream,
            reader,
            db: opened_db,
            level: user.level,
            address,
            timeout: 0,
            set_timeout: self.config.timeout.clone(),
        };
        self.new_client(client,db_path);
        Ok(())
    }

    pub fn refresh(&mut self) -> Result<()> {
        let mut should_close: HashMap<String, ()> = HashMap::new();
        let mut dead_client: Vec<String> = Vec::new();
        for (address, (path, thread_handle)) in &self.clients {
            if thread_handle.is_finished() {
                dead_client.push(address.clone());
                should_close.insert(path.clone(),());
                continue;
            } else {
                should_close.remove(path);
            }
        }
        for address in &dead_client {
            let _ = &self.clients.remove(address);
        }
        for (path,()) in &should_close {
            self.dbs.remove(path);
        }
        Ok(())
    }

    fn new_client(&mut self, mut client: Client, datafile_index: String) {
        let address = client.address.to_string();
        let thread_handle = thread::spawn(move || {
            client.handle_client();
        });

        let _ = &self.clients.insert(
            address,
            (datafile_index, thread_handle)
        );
    }

    fn send_error(stream: &mut TcpStream, err: ConnectError) -> Result<()> {
        let (buf, _) = Message::new(err).as_bytes()?;
        stream.write(&buf.as_slice())?;
        stream.shutdown(Shutdown::Both)?;
        Ok(())
    }

    fn send_reply(stream: &mut TcpStream) -> Result<()> {
        let (buf, _) = Message::new(ConnectReply::Success).as_bytes()?;
        stream.write(&buf.as_slice())?;
        Ok(())
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

    fn compare(&mut self, path: String, stream: &mut TcpStream) -> Result<DataPath> {
        for (key, db) in &self.dbs {
            let exists = match is_same_file(&key, &path) {
                Ok(b) => b,
                Err(e) => {
                    Self::send_error(stream, ConnectError::ServerError)?;
                    return Err(RorError::IOError(e));
                },
            };
            if exists {
                let opened_db = Arc::clone(&db);
                return Ok(DataPath::Exists(opened_db));
            }
        }
        return Ok(DataPath::None)
    }

} 

pub struct Client {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    db: Arc<Mutex<DataStore>>,
    level: String,
    address: SocketAddr,
    timeout: u64,
    set_timeout: u64,
}

impl Client {
    fn handle_client(&mut self) {
        loop {
            thread::sleep(time::Duration::from_secs(1 as u64));

            if &self.timeout >= &self.set_timeout {
                output_prompt(format!("Client [{0}] activity timeout", &self.address));
                let _ = &self.stream.shutdown(Shutdown::Both);
                break;
            }
            match &&self.accept_request() {
                Ok(()) => continue,
                Err(RorError::Disconnect) => {
                    output_prompt(format!("Client [{0}] disconnected", &self.address));
                    let _ = &self.stream.shutdown(Shutdown::Both);
                    break;
                },
                Err(RorError::KvError(e)) => output_prompt(format!("An error occurred on client [{0}], buffer flushed and error message sent. {1}",&self.address,e)),
                Err(e) => {
                    if let Err(_) = &self.stream.shutdown(Shutdown::Both) {
                        output_prompt(format!("Client [{0}] unable to properly disconnect, thread has been forcibly closed",&self.address));
                    }
                    output_prompt(format!("An error occurred on client [{0}]. It may be fatal, the connection was forcibly terminated. {1}",&self.address,e));
                    break;
                }
            }
        }
    }

    fn accept_request(&mut self) -> Result<()> {
        let mut size_buffer = [0 as u8; USIZE_SIZE];
        match self.reader.read_exact(&mut size_buffer) {
            Ok(_) => self.timeout = 0,
            Err(e) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    self.timeout += 1;
                    return Ok(());
                }
                return Err(RorError::IOError(e));
            }
        }
        let body_size = usize::from_be_bytes(size_buffer);
        let mut body_buffer = vec![0; body_size];
        match self.reader.read(&mut body_buffer) {
            Ok(_) => (),
            Err(_) => return Ok(()),
        }
        let op: OperateRequest = match bincode::deserialize(&body_buffer) {
            Ok(r) => r,
            Err(_) => {
                let msg = Message::new(OperateResult::Failure);
                let (buf,_) = msg.as_bytes()?; 
                self.stream.write(&buf)?;
                self.stream.flush()?;
                return Ok(());
            }
        };

        match self.match_command(op) {
            Ok(r) => {
                let msg = Message::new(r);
                let (buf,_) = msg.as_bytes()?; 
                self.stream.write(&buf)?;
                self.stream.flush()?;
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }

    fn match_command(&mut self, command: OperateRequest) -> Result<OperateResult> {
        match command {
            OperateRequest::Open { path } => {
                todo!()
            }
            OperateRequest::Get { key } => {
                match self.db.lock().unwrap().get(key) {
                    Ok(v) => {
                        return Ok(OperateResult::Found(v));
                    }
                    Err(KvError::KeyNotFound(_)) => return Ok(OperateResult::KeyNotFound),
                    Err(e) => return Err(RorError::KvError(e)),
                }
            }
            OperateRequest::Delete { key } => {
                if self.level != "2" && self.level != "3" {
                    return Ok(OperateResult::PermissionDenied);
                }
                match self.db.lock().unwrap().delete(key) {
                    Ok(_) => {
                        return Ok(OperateResult::Success);
                    }
                    Err(KvError::KeyNotFound(_)) => return Ok(OperateResult::KeyNotFound),
                    Err(e) => return Err(RorError::KvError(e)),
                }
            }
            OperateRequest::Add { key, value } => {
                if self.level != "1" && self.level != "2" && self.level != "3" {
                    return Ok(OperateResult::PermissionDenied);
                }
                match self.db.lock().unwrap().add(key,value) {
                    Ok(_) => {
                        return Ok(OperateResult::Success);
                    }
                    Err(e) => return Err(RorError::KvError(e)),
                }
            }
            OperateRequest::CreateUser { name, password, level } => {
                if self.level != "3" {
                    return Ok(OperateResult::PermissionDenied);
                }

                match User::register(                    
                    name,
                    password,
                    level
                ) {
                    Ok(_) => return Ok(OperateResult::Success),
                    Err(e) => {
                        output_prompt(format!("Unable to create new user for client [{0}], {1}", self.address, e));
                        return Ok(OperateResult::Failure);
                    }
                }
            },
            OperateRequest::DeleteUser { name } => {
                if self.level != "3" {
                    return Ok(OperateResult::PermissionDenied);
                }

                match User::delete(name.clone()) {
                    Ok(_) => Ok(OperateResult::Success),
                    Err(e) => {
                        output_prompt(format!("Unable to delete user '{0}' for client [{1}], {2}", name, self.address, e));
                        return Ok(OperateResult::Failure);
                    }
                }
            }
            OperateRequest::GetType { key } => {
                match self.db.lock().unwrap().get(key) {
                    Ok(v) => {
                        return Ok(OperateResult::Type(DataStore::type_of(v)));
                    }
                    Err(KvError::KeyNotFound(_)) => return Ok(OperateResult::KeyNotFound),
                    Err(e) => return Err(RorError::KvError(e)),
                }
            }
            OperateRequest::Compact => {
                if self.level != "1" && self.level != "2" && self.level != "3" {
                    return Ok(OperateResult::PermissionDenied);
                }
                match self.db.lock().unwrap().compact() {
                    Ok(_) => {
                        return Ok(OperateResult::Success);
                    }
                    Err(e) => return Err(RorError::KvError(e)),
                }
            },
            OperateRequest::Quit => {
                return Err(RorError::Disconnect);
            },
        }
    }
}

#[derive(Deserialize,Serialize,Clone)]
struct Config {
    name: String,
    ip: String,
    port: String,
    data_path: String,
    timeout: u64,
    repl: bool,
    local_user: String,
    default_db: String,
    auto_refresh: u32,
}

impl Config {
    fn default() -> Self {
        Config {
            name: "Default server".to_string(),
            ip: "127.0.0.1".to_string(),
            port: "11451".to_string(),
            data_path: "./data/".to_string(),
            timeout: 300,
            repl: false,
            local_user: String::new(),
            default_db: String::new(),
            auto_refresh: 20,
        }
    }
    pub fn get_server() -> Result<Self> {
        let mut file = File::open("config/server.toml")?;
        let mut c = String::new();
        file.read_to_string(&mut c)?;
        let config: Config  = toml::from_str(c.as_str())?;
        Ok(config)
    }
}

fn output_prompt<T: std::fmt::Display>(content: T) {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("[{0}] {1}",time.yellow(),content);
}