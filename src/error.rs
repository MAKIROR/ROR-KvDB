extern crate quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum KvError {
        IoError( err: std::io::Error ) {
            from()
            display("IO error: {}", err)
            source(err)
        }
        InvalidPath( path: String ) {
            display("Invalid Path \"{}\"", path)
        }
        KeyNotFound( key: String ) {
            display("Key not found: \"{}\"", key)
        }
        SerdeError( err: serde_json::Error ) {
            from()
            display("Serde Json Error: {}", err)
            source(err)
        }
        Other(err: Box<dyn std::error::Error>) {
            cause(&**err)
        }
    }
}