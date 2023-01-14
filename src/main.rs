use std::{
    env,
    io::{self, Write},
};
use rdb::RorDb;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let p = input_path();
        args.push(p);
    }
    let db = match RorDb::open(args[1].clone()) {
        Ok(db) => db,
        Err(e) => {
            println!("{}", e);
            std::process::exit(0);
        }
    };
    RorDb::run(db);
}

fn input_path() -> String {
    print!("Please enter your datafile path\n(If the file does not exist, it will be create automatically): ");
    io::stdout().flush().unwrap();
    let mut path = String::new();
    std::io::stdin().read_line(&mut path).unwrap();    
    if path.trim() != "" {
        return path.trim().to_string();
    } else {
        input_path()
    }
}