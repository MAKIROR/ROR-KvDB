use std::{
    io::Read,
    string::String,
    fs::File,
};
use regex::Regex;
use super::user_error::{UserError,Result};
use toml;

pub enum Verify {
    Correct,
    Wrong,
}

pub struct User {
    uid: String,
    name: String,
    password: String,
    level: String,
}

impl User {
    pub fn new( name: String, password: String, level: String ) -> Result<Self> {
        /*
        let password_regex = Regex::new(r"^[a-zA-Z0-9_-]{4,16}$")?;
        if !password_regex.is_match(&password) {
            return Err(UserError::PassWordFormatError(password));
        }
        let name_len = name.chars().count();
        if name_len < 2 || name_len > 20 {
            return Err(UserError::NameLengthError(name_len));
        }

        
        match level.as_str() {
            "0"
            | "1"
            | "2" => return Ok(User{name,password,level}),
            _ => return Err(UserError::UnknownLevel(level)),
        }     
        */
        todo!()
    }
    pub fn register(&self) {
        todo!()
    }
}

fn get_users_dir() -> Result<String> {
    let mut file = match File::open("config/users.toml") {
        Ok(f) => f,
        Err(_) => return Ok("./users".to_string()),
    };
    let mut c = String::new();
    file.read_to_string(&mut c)?;
    let config: String = toml::from_str(c.as_str())?;
    Ok(config)
}