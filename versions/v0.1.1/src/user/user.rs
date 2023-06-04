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
    name: String,
    password: String,
    pub level: String,
}

impl User {
    pub fn register( name: String, password: String, level: String ) -> Result<()> {
        let password_regex = Regex::new(r"^[a-zA-Z0-9_-]{4,16}$")?;
        if !password_regex.is_match(&password) {
            return Err(UserError::PassWordFormatError(password));
        }
        let name_len = name.chars().count();
        if name_len < 2 || name_len > 20 {
            return Err(UserError::NameLengthError(name_len));
        }
        let mut user = match level.as_str() {
            "0" | "1" | "2" | "3" => User {
                name: name,
                password: password,
                level: level,
            },
            _ => return Err(UserError::UnknownLevel(level)),
        };

        let config_path = USER_PATH.clone();
        let config_max = *USER_MAX;

        let original = fs::read_to_string(&config_path)?;
        let mut data: Vec<User> = serde_json::from_str(&original)?;
        if data.len() > config_max.into() && config_max != 0 {
            return Err(UserError::UserLimit);
        }
        if let Ok(_) = Self::search(&data,user.name.clone()) {
            return Err(UserError::UserNameExists(user.name));
        }
        user.encode();
        data.push(user);
        let json = serde_json::to_string(&data)?;
        let mut file = File::create(&config_path)?;
        write!(file, "{}", json)?;
        Ok(())
    }

    pub fn login( name: String, password: String ) -> Result<Self> {
        let config_path = USER_PATH.clone();
        let str_data = fs::read_to_string(&config_path)?;
        let data: Vec<User> = serde_json::from_str(&str_data)?;
        let user = match Self::search(&data,name) {
            Ok(user) => user,
            Err(e) => return Err(e),
        };
        if password == user.password {
            return Ok(user);
        } else {
            return Err(UserError::WrongPassWord);
        }
    }

    pub fn delete( name: String ) -> Result<()> {
        let config_path = USER_PATH.clone();
        let str_data = fs::read_to_string(&config_path)?;
        let mut data: Vec<User> = serde_json::from_str(&str_data)?;
        for (i, user) in data.iter().enumerate() {
            if user.name == name {
                data.remove(i);
                
                let json = serde_json::to_string(&data)?;
                let mut file = File::create(&config_path)?;
                write!(file, "{}", json)?;

                return Ok(())
            }
        }
        Err(UserError::UserNotFound(name))
    }
    
    fn search(data: &Vec<User>, name: String) -> Result<Self> {
        for u in data {
            if u.name == name {
                let mut user = u.clone();
                user.decode()?;
                return Ok(user);
            }
        }
        Err(UserError::UserNotFound(name))
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

#[derive(Deserialize,Serialize)]

pub struct Config {
    path: String,
    user_max: u16,
}

impl Config {
    pub fn default() -> Self {
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