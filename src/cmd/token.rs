pub enum Token {
    DataType(DataType),
    Command(Command),
    Identifier(String),
    Number(String),
    Bool(bool),
}

pub enum DataType {
    Null,
    Bool,
    Int,
    Long,
    Float,
    Double,
    Char,
    String,
    Array
}

pub enum Command {
    Open,
    Add,
    Delete,
    Compact,
    Get,
    TypeOf,
    List,
    User,
    Quit
}

pub trait StringExt {
    fn as_datatype(&self) -> Option<DataType>;
    fn as_command(&self) -> Option<Command>;
    fn as_bool(&self) -> Option<bool>;
}

impl StringExt for String {
    fn as_datatype(&self) -> Option<DataType> {
        match self.as_str() {
            "null" => Some(DataType::Null),
            "bool" => Some(DataType::Bool),
            "i32" | "int" => Some(DataType::Int),
            "i64" | "long" => Some(DataType::Long),
            "f32" | "float" => Some(DataType::Float),
            "f64" | "double" => Some(DataType::Double),
            "char" => Some(DataType::Char),
            "string" => Some(DataType::String),
            "array" => Some(DataType::Array),
            _ => None
        }
    }

    fn as_command(&self) -> Option<Command> {
        match self.as_str() {
            "open" => Some(Command::Open),
            "add" => Some(Command::Add),
            "delete" => Some(Command::Delete),
            "compact" => Some(Command::Compact),
            "get" => Some(Command::Get),
            "typeof" => Some(Command::TypeOf),
            "list" => Some(Command::List),
            "user" => Some(Command::User),
            "quit" => Some(Command::Quit),
            _ => None
        }
    }

    fn as_bool(&self) -> Option<bool> {
        if self.to_uppercase() == "TRUE" {
            return Some(true)
        } else if self.to_uppercase() == "FALSE" {
            return Some(false)
        }
        None
    }
}


