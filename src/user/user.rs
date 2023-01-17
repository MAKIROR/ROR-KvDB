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
use base64;
use toml;

#[derive(Debug)]
pub enum Verify {
    Correct,
    WrongPassWord,
    UserNotFound,
}

#[derive(Deserialize,Serialize,Clone)]
pub struct User {
    uid: String,
    name: String,
    password: String,
    level: String,
}

impl User {
    pub fn new( worker_id: i64, data_center_id: i64, name: &str, password: &str, level: &str ) -> Result<Self> {
        let password_regex = Regex::new(r"^[a-zA-Z0-9_-]{4,16}$")?;
        if !password_regex.is_match(&password) {
            return Err(UserError::PassWordFormatError(password.to_string()));
        }
        let name_len = name.chars().count();
        if name_len < 2 || name_len > 20 {
            return Err(UserError::NameLengthError(name_len));
        }
        let uid = Snowflake::new(worker_id,data_center_id)?.generate()?;
        match level {
            "0"
            | "1"
            | "2" => return Ok(User{
                uid,
                name: name.to_string(),
                password: password.to_string(),
                level: level.to_string(),
            }),
            _ => return Err(UserError::UnknownLevel(level.to_string())),
        } 
    }
    pub fn register(&self) -> Result<()> {
        let mut user = self.clone();
        let name_len = user.name.chars().count();
        let password_regex = Regex::new(r"^[a-zA-Z0-9_-]{4,16}$")?;
        if name_len < 2 || name_len > 20 {
            return Err(UserError::NameLengthError(name_len));
        } else if !password_regex.is_match(&user.password) {
            return Err(UserError::PassWordFormatError(user.password));
        }

        if let Ok(_) = Self::search(user.name.clone()) {
            return Err(UserError::UserNameExists(user.name));
        }
        
        let config = Config::get_config()?;
        if Self::count_users()? > config.user_max {
            return Err(UserError::UserLimit);
        }
        let path_slice = Path::new(&config.path);
        if !path_slice.exists() {
            File::create(&config.path)?;
        } 
        let config = Config::get_config()?;
        let original = fs::read_to_string(&config.path)?;
        let mut data: Vec<User> = Vec::new();
        if original.len() != 0 {
            data = serde_json::from_str(&original)?;
        }
        user.encode();
        data.push(user);
        let json = serde_json::to_string(&data)?;
        let mut file = File::create(&config.path)?;
        write!(file, "{}", json)?;
        Ok(())
    }
    pub fn login( name: &str, password: &str ) -> Result<Verify> {
        let user = match Self::search(name.to_string()) {
            Ok(user) => user,
            Err(UserError::UserNotFound(_)) => return Ok(Verify::UserNotFound),
            Err(e) => return Err(e),
        };
        if password == user.password {
            return Ok(Verify::Correct);
        } else {
            return Ok(Verify::WrongPassWord);
        }
    }
    
    fn search(name: String) -> Result<User> {
        let config = Config::get_config()?;
        let str_data = fs::read_to_string(&config.path)?;
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
        let config = Config::get_config()?;
        let str_data = fs::read_to_string(&config.path)?;
        let data: Vec<User> = serde_json::from_str(&str_data)?;
        Ok(data.len().try_into()?)
    }

    fn encode(&mut self) {
        self.password = base64::encode(self.password.clone());
    }
    fn decode(&mut self) -> Result<()> {
        let bytes = base64::decode(self.password.clone())?;
        self.password = std::str::from_utf8(&bytes)?.to_string();
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
    fn get_config() -> Result<Self> {
        let mut file = match File::open("config/users.toml") {
            Ok(f) => f,
            Err(_) => return Ok(Self::default()),
        };
        let mut c = String::new();
        file.read_to_string(&mut c)?;
        let config: Config = toml::from_str(c.as_str())?;
        Ok(config)
    }
}
