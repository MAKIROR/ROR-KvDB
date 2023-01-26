use rdb::RemoteRepl;

#[test]
fn run_repl() {
    let mut _r = RemoteRepl::new("127.0.0.1","11451","makiror","114514","test.data").unwrap();
}