use std::{
    io::{
        Read,
        Write,
    },
    string::String,
    fs::{
        self,
        File,
    },
    path::Path,
};
use regex::Regex;
use super::user_error::{UserError,Result};
use super::snowflake::Snowflake;
use serde::{Serialize,Deserialize};
use base64::{Engine as _, engine::general_purpose};
use lazy_static::lazy_static;
use toml;

lazy_static! {
    static ref USER_PATH: String = Config::get_config().path;
    static ref USER_MAX: u16 = Config::get_config().user_max;
}

#[derive(Deserialize,Serialize,Clone)]
pub struct User {
    uid: String,
    name: String,
    password: String,
    pub level: String,
}

impl User {
    pub fn register( worker_id: i64, data_center_id: i64, name: &str, password: &str, level: &str ) -> Result<()> {
        let config_path = USER_PATH.clone();
        let config_max = *USER_MAX;

        let path_slice = Path::new(&config_path);
        if !path_slice.exists() {
            let mut f = File::create(&config_path)?;
            write!(f, "{}", "[]")?;
        } 

        let password_regex = Regex::new(r"^[a-zA-Z0-9_-]{4,16}$")?;
        if !password_regex.is_match(&password) {
            return Err(UserError::PassWordFormatError(password.to_string()));
        }
        let name_len = name.chars().count();
        if name_len < 2 || name_len > 20 {
            return Err(UserError::NameLengthError(name_len));
        }
        let uid = Snowflake::new(worker_id,data_center_id)?.generate()?;
        let mut user = match level {
            "0"
            | "1"
            | "2"
            | "3" => User{
                uid,
                name: name.to_string(),
                password: password.to_string(),
                level: level.to_string(),
            },
            _ => return Err(UserError::UnknownLevel(level.to_string())),
        };

        if let Ok(_) = Self::search(user.name.clone()) {
            return Err(UserError::UserNameExists(user.name));
        }
        
        if Self::count_users()? > config_max {
            return Err(UserError::UserLimit);
        }
        let original = fs::read_to_string(&config_path)?;
        let mut data: Vec<User> = Vec::new();
        if original.len() != 0 {
            data = serde_json::from_str(&original)?;
        }
        user.encode();
        data.push(user);
        let json = serde_json::to_string(&data)?;
        let mut file = File::create(&config_path)?;
        write!(file, "{}", json)?;
        Ok(())
    }
    pub fn login( name: String, password: String ) -> Result<Self> {
        let user = match Self::search(name) {
            Ok(user) => user,
            Err(UserError::UserNotFound(name)) => return Err(UserError::UserNotFound(name)),
            Err(e) => return Err(e),
        };
        if password == user.password {
            return Ok(user);
        } else {
            return Err(UserError::WrongPassWord);
        }
    }
    
    fn search(name: String) -> Result<User> {
        let config_path = USER_PATH.clone();
        
        let str_data = fs::read_to_string(&config_path)?;
        let data: Vec<User> = serde_json::from_str(&str_data)?;
        for u in data {
            if u.name == name {
                let mut user = u.clone();
                user.decode()?;
                return Ok(user);
            }
        }
        Err(UserError::UserNotFound(name))
    }
    fn count_users() -> Result<u16> {
        let config_path = USER_PATH.clone();

        let str_data = fs::read_to_string(&config_path)?;
        let data: Vec<User> = serde_json::from_str(&str_data)?;
        Ok(data.len().try_into()?)
    }
    fn encode(&mut self) {
        self.password = general_purpose::STANDARD_NO_PAD.encode(self.password.clone());
    }
    fn decode(&mut self) -> Result<()> {
        let bytes = general_purpose::STANDARD_NO_PAD.decode(self.password.clone())?;
        self.password = std::str::from_utf8(&bytes)?.to_string();
        Ok(())
    }
    pub fn test_file() -> Result<()> {
        let config_path = USER_PATH.clone();

        let path_slice = Path::new(&config_path);
        if !path_slice.exists() {
            let mut f = File::create(&config_path)?;
            write!(f, "{}", "[]")?;
            return Ok(());
        }
        Ok(())
    }
}

#[derive(Deserialize)]
struct Config {
    path: String,
    user_max: u16,
}

impl Config {
    fn default() -> Self {
        Config {
            path: "users.json".to_string(),
            user_max: 50,
        }
    }
    fn get_config() -> Self {
        let mut file = match File::open("config/users.toml") {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };
        let mut c = String::new();
        match file.read_to_string(&mut c) {
            Ok(_) => (),
            Err(_) => return Self::default(),
        };
        let config: Config = match toml::from_str(c.as_str()) {
            Ok(c) => c,
            Err(_) => return Self::default(),
        };
        config
    }
}