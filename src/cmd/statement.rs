use super::token::token::*;

pub trait TokenExt {
    fn to_value(&self) -> Option<Value>;
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
        value: Value
    },
    Delete { key: String },
    Compact,
    Get { key: String },
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
    Null,
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
    name: String,
    password: String,
    level: String,
}