use rorkv::{self,DataStore};

fn main() {
    DataStore::open("test.data".to_string());
}