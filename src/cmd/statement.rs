use super::token::token::*;
use std::fmt;

pub trait TokenExt {
    fn to_value(&self) -> Option<ValueP>;
}

#[derive(Clone, Debug)]
pub enum ValueType {
    Null,
    Bool,
    Int32,
    Int64,
    Float32,
    Float64,
    Char,
    String,
    Array(Box<ValueType>)
}

impl TokenExt for Option<Token> {
    fn to_value(&self) -> Option<ValueP> {
        match self {
            Some(Token::Identifier(s)) => Some(ValueP::Identifier(s.clone())),
            Some(Token::Number(n)) => Some(ValueP::Number(n.clone())),
            Some(Token::Bool(b)) => Some(ValueP::Bool(*b)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Open { file: String },
    Add {
        key: String,
        value: ValueP,
        datatype: ValueType
    },
    Delete { key: String },
    Get { key: String },
    Compact,
    TypeOf { key: String },
    List { list: List },
    User { cmd: UserCmd },
    Quit
}

#[derive(Debug, Clone)]
pub enum ValueP {
    Identifier(String),
    Number(String),
    Bool(bool),
    Array(Box<Vec<ValueP>>)
}

impl ValueP {
    pub fn get_str(&self) -> String {
        match self {
            Self::Identifier(s) 
            | Self::Number(s) => s.clone(),
            Self::Bool(b) => b.to_string(),
            Self::Array(a) => format!("{:?}", a),
        }
    }
}

#[derive(Clone, Debug)]
pub enum List {
    Values,
    Entries
}

#[derive(Clone, Debug)]
pub enum UserCmd {
    Create { info: UserInfo },
    Delete { name: String }
}

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub name: String,
    pub password: String,
    pub level: String,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Null => write!(f, "null"),
            ValueType::Bool => write!(f, "bool"),
            ValueType::Int32 => write!(f, "int"),
            ValueType::Int64=> write!(f, "long"),
            ValueType::Float32 => write!(f, "float"),
            ValueType::Float64 => write!(f, "double"),
            ValueType::Char => write!(f, "char"),
            ValueType::String => write!(f, "string"),
            ValueType::Array(a) => write!(f, "{:?}", a)
        }
    }
}

impl fmt::Display for ValueP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueP::Identifier(s) | ValueP::Number(s) => write!(f, "{}", s),
            ValueP::Bool(b) => write!(f, "{}", b.to_string()),
            ValueP::Array(a) => write!(f, "{:?}", a),
        }
    }
}