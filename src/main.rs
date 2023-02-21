use std::{
    io::{self, Write},
};

use clap::{arg, Command};

use rdb::{Server,LocalRepl,RemoteRepl};

fn main() {
    let matches = Command::new("ROR Key-Value Database")
        .version("0.1.0")
        .author("MAKIROR")
        .about("ROR Key-Value Database")
        .subcommand(
            Command::new("local")
            .about("Start the local database")
            .arg(arg!(-p --path <Path> "Datafile path")),
        )
        .subcommand(
            Command::new("server")
            .about("Start the database server")
            .subcommand(
                Command::new("init")
                .about("Initialize server")
            )
        )
        .subcommand(
            Command::new("connect")
            .about("Connect to remote database")
            .arg(arg!(-i --ip <VALUE> "IP"))
            .arg(arg!(-p --port <VALUE> "Port"))
            .arg(arg!(-u --user <VALUE> "User Info (username@password)"))
            .arg(arg!(-f --file <VALUE> "Datafile"))
        )
    .get_matches();
    match matches.subcommand() {
        Some(("server", sub_m)) => {
            if let Some(("init",_)) = sub_m.subcommand() {
                Server::init().unwrap();
            }
            let mut s = Server::new();
            s.start().unwrap();
        }
        Some(("local", sub_m)) => {
            if let Some(path) = sub_m.get_one::<String>("path") {
                let mut repl = LocalRepl::open(&path.as_str()).unwrap();
                repl.run();
            } else {
                let path = input_something("datafile path");
                let mut repl = LocalRepl::open(path.as_str()).unwrap();
                repl.run();
            }
        }
        Some(("connect", sub_m)) => {
            let ip = match sub_m.get_one::<String>("ip") {
                Some(ip) => ip.clone(),
                None => input_something("ip"),
            };
            let port = match sub_m.get_one::<String>("port") {
                Some(port) => port.clone(),
                None => input_something("port"),
            };
            let (username,password) = match sub_m.get_one::<String>("user") {
                Some(u) => {
                    let user: Vec<&str> = u.split("@").collect();
                    if user.len() != 2 {
                        let username = input_something("username");
                        let password = input_something("password");
                        (username.to_string(),password.to_string())
                    } else {
                        (user[0].to_string(),user[1].to_string())
                    }
                },
                None => {
                    let username = input_something("username");
                    let password = input_something("password");
                    (username,password)
                },
            };
            let db_path = match sub_m.get_one::<String>("file") {
                Some(ip) => ip.clone(),
                None => input_something("datafile path"),
            };
            let mut repl = match RemoteRepl::new(ip,port,username,password,db_path) {
                Ok(r) => r,
                Err(e) => {
                    println!("{}",e);
                    std::process::exit(0);
                }
                
            };
            repl.run();
        }
        _ => {
            println!("Unable to start, starting 'local'");
            let path = input_something("datafile path");
            let mut repl = LocalRepl::open(path.as_str()).unwrap();
            repl.run();
        },
    };
    
}

fn input_something(p: &str) -> String {
    print!("Please enter your {0}: ",p);
    io::stdout().flush().unwrap();
    let mut parameter = String::new();
    std::io::stdin().read_line(&mut parameter).unwrap();    
    if parameter.trim() != "" {
        return parameter.trim().to_string();
    } else {
        input_something(p)
    }
}