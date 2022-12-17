pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

impl Command {
    fn set( key: String,value: String ) -> Command {
        Command::Set { key,value }
    }
    fn get( key: String ) -> Command {
        Command::Get { key }
    }
    fn remove( key: String ) -> Command {
        Command::Remove { key }
    }
}