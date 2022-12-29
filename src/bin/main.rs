use rorkv::{self,DataStore};

fn main() {
    DataStore::open("test.json".to_string());
}