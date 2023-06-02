use super::token::token::*;

pub trait TokenExt {
    fn to_value(&self) -> Option<Value>;
}

#[derive(Clone, Debug)]
pub enum ValueType {
    Null,
    Bool,
    Int,
    Long,
    Float,
    Double,
    Char,
    String,
    Array(Box<ValueType>)
}

impl TokenExt for Option<Token> {
    fn to_value(&self) -> Option<Value> {
        match self {
            Some(Token::Identifier(s)) => Some(Value::Identifier(s.clone())),
            Some(Token::Number(n)) => Some(Value::Number(n.clone())),
            Some(Token::Bool(b)) => Some(Value::Bool(*b)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Open { file: String },
    Add {
        key: String,
        value: Value,
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
pub enum Value {
    Identifier(String),
    Number(String),
    Bool(bool),
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