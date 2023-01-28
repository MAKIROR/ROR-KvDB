use rdb::RemoteRepl;
use rdb::OperateRequest;
use std::net::{TcpListener, TcpStream, Shutdown, SocketAddr};

#[test]
fn run_repl() {
    let mut _r = RemoteRepl::new("127.0.0.1","11451","makiror","114514","test.data").unwrap();
}