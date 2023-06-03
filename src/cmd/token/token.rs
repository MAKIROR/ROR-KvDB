use std::fmt;
use super::{
    command::*,
    symbol::*,
    datatype::*,
    arg::*,
};

#[derive(Clone, Debug)]
pub enum Token {
    DataType(DataType),
    Command(Command),
    Symbol(Symbol),
    Arg(Arg),
    Identifier(String),
    Number(String),
    Bool(bool),
}

pub trait StringExt {
    fn as_datatype(&self) -> Option<DataType>;
    fn as_command(&self) -> Option<Command>;
    fn as_symbol(&self) -> Option<Symbol>;
    fn as_arg(&self) -> Option<Arg>;
    fn as_bool(&self) -> Option<bool>;
}

impl StringExt for String {
    fn as_datatype(&self) -> Option<DataType> {
        match self.as_str() {
            "null" => Some(DataType::Null),
            "bool" => Some(DataType::Bool),
            "i32" | "int" => Some(DataType::Int32),
            "i64" | "long" => Some(DataType::Int64),
            "f32" | "float" => Some(DataType::Float32),
            "f64" | "double" => Some(DataType::Float64),
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
            "create" => Some(Command::Create),
            _ => None
        }
    }

    fn as_symbol(&self) -> Option<Symbol> {
        match self.as_str() {
            "," => Some(Symbol::Comma),
            "." => Some(Symbol::Dot),
            "*" => Some(Symbol::Asterisk),
            "+" => Some(Symbol::Plus),
            "-" => Some(Symbol::Minus),
            "/" => Some(Symbol::Slash),
            "%" => Some(Symbol::Percent),
            "=" => Some(Symbol::Equal),
            "!=" => Some(Symbol::NotEqual),
            "<" => Some(Symbol::LessThan),
            ">" => Some(Symbol::GreaterThan),
            "<=" => Some(Symbol::LessThanOrEqual),
            ">=" => Some(Symbol::GreaterThanOrEqual),
            "(" => Some(Symbol::LeftParen),
            ")" => Some(Symbol::RightParen),
            ";" => Some(Symbol::Semicolon),
            "[" => Some(Symbol::LeftBracket),
            "]" => Some(Symbol::RightBracket),
            _ => None,
        }
    }

    fn as_arg(&self) -> Option<Arg> {
        match self.as_str() {
            "values" => Some(Arg::Values),
            "entries" => Some(Arg::Entries),
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

pub trait SqlCharExt {
    fn is_symbol(&self) -> bool;
}

impl SqlCharExt for char {
    fn is_symbol(&self) -> bool {
        if to_symbol(&self.to_string().as_str()).is_some() {
            return true
        }
        false
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Command(command) => write!(f, "{}", command),
            Token::DataType(datatype) => write!(f, "{}", datatype),
            Token::Symbol(symbol) => write!(f, "{}", symbol),
            Token::Arg(arg) => write!(f, "{}", arg),
            Token::Identifier(identifier) => write!(f, "{}", identifier),
            Token::Number(num) => write!(f, "{}", num),
            Token::Bool(bool) => {
                match bool {
                    true => write!(f, "TRUE"),
                    false => write!(f, "FALSE"),
                }
            }
        }
    }
}