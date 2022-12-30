use std::{
    path::PathBuf,
    collections::HashMap,
};

pub struct DBManager {
    list: HashMap<String,DataStore>,
    location: PathBuf,
}