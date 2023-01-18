use std::{
    io::{
        Read,
        Write,
    },
    net::{SocketAddr, TcpListener, TcpStream, Shutdown},
    fs::File,
    collections::HashMap,
    sync::Mutex,
    thread,
};
use super::{
    error::{RorError,Result},
    store::kv::{DataStore,Value},
    user::{
        user::User,
    },
};
use serde::{Serialize,Deserialize};
use same_file::is_same_file;
use bincode;

#[derive(Serialize, Deserialize)]
pub struct ConnectRequest {
    db_path: String,
    user_name: String,
    user_password: String,
}

#[derive(Serialize, Deserialize)]
pub enum OperateRequest {
    Get { key: String },
    Add { key: String, value: Value },
    Delete { key: String },
}

pub struct Client {
    address: SocketAddr,
    stream: TcpStream,
    db_path: String,
    user: User,
}

pub struct Server {
    config: Config,
    clients: Vec<Client>,
    dbs: HashMap<String,Mutex<DataStore>>
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
            dbs: HashMap::new(),
        };
    }
    pub fn start(&mut self) -> Result<()> {
        let address = self.config.ip.clone() + ":" + &self.config.port.clone();
        let listener = TcpListener::bind(address)?;
        loop {
            let (mut stream, address) = listener.accept()?;

            let mut head_buffer: Vec<u8> = Vec::new();
            stream.read(&mut head_buffer)?;
            let head: ConnectRequest = bincode::deserialize(&head_buffer)?;
            let user = match User::login(head.user_name,head.user_password) {
                Ok(u) => u,
                Err(e) => return Err(RorError::UserError(e)),
            };
            let mut db_path = self.config.data_path.clone() + &head.db_path;
            let mut should_open = true;
            for (key, _) in &self.dbs {
                let exists = is_same_file(&key, &db_path)?;
                if exists {
                    db_path = key.clone();
                    should_open = false;
                    break;
                }
            }
            if should_open {
                self.open_new_db(db_path.clone())?;
            }
            let client = Client {
                stream,
                address,
                db_path,
                user,
            };
            
            thread::spawn(move|| {
                Self::handle_client(client);
            });
            //todo
        }
        Ok(())
    }
    fn handle_client(mut client: Client) -> Result<()> {

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
        client.stream.shutdown(Shutdown::Both)?;
        Ok(())
    }
    fn open_new_db(&mut self, path: String) -> Result<()> {
        let db = DataStore::open(path.as_str())?;
        self.dbs.insert( db.path.clone(), Mutex::new(db) );
        Ok(())
    }
    fn match_command(&mut self, client: &Client, command: Vec<&str>) -> Result<()> {
        if let Some(mut db) = self.dbs.get(&client.db_path) {
            todo!()
        } else {
            return Err(RorError::DataFileNotFound(client.db_path.clone()));
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
