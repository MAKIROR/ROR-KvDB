use std::{
    env,
    io::{self, Write},
    path::Path,
};
use rorkv::{RorDb,DataStore,KvError,Result};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        loop {
            print!("Please enter your database file path\n(If the file does not exist, it will be create automatically): ");
            io::stdout().flush().unwrap();
            let mut path = String::new();
            std::io::stdin().read_line(&mut path).unwrap();    
            if path.trim() != "" {
                args.push(path.trim().to_string());  
                break;
            }
        }
    }
    let db = DataStore::open(args[1].clone()).unwrap();
    RorDb::run(RorDb{database: db});
}