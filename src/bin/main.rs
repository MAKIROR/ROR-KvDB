use rorkv::{DataStore,KvError};

fn main() {
    let mut db = match DataStore::open("test.data".to_string()) {
        Ok(db) => db,
        Err(e) => {
            println!("{}",e.to_string());
            return;
        }
    };
    match db.get("test".to_string()) {
        Ok(vec) => println!("0"),
        Err(KvError::KeyNotFound(_key)) => println!("key not found"),
        Err(e) => println!("{}",e.to_string()),
    }
}