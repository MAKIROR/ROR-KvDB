use std::{vec::IntoIter, iter::Peekable};
use super::{
    token::{
        token::*,
        command::Command,
        symbol::Symbol,
        datatype::DataType,
        arg::Arg,
    },
    lexer::lex,
    statement::*,
    cmd_error::{CmdError, Result},
};

pub struct Parser {
    iter: Peekable<IntoIter<Token>>
}

impl Parser {
    pub fn new() -> Self {
        Self {
            iter: Vec::new().into_iter().peekable()
        }
    }

    pub fn parse(&mut self, s: &str) -> Result<Statement> {
        let tokens = lex(s);
        self.iter = tokens.clone().into_iter().peekable();
        let statement = match tokens.get(0) {
            Some(Token::Command(Command::Open)) => self.parse_open()?,
            Some(Token::Command(Command::Add)) => self.parse_add()?,
            Some(Token::Command(Command::Delete)) => self.parse_delete()?,
            Some(Token::Command(Command::Get)) => self.parse_get()?,
            Some(Token::Command(Command::TypeOf)) => self.parse_typeof()?,
            Some(Token::Command(Command::User)) => self.parse_user()?,
            Some(Token::Command(Command::List)) => self.parse_list()?,
            Some(Token::Command(Command::Compact)) => Statement::Compact,
            Some(Token::Command(Command::Quit)) => Statement::Quit,
            Some(t) => return Err(CmdError::UnexpectedToken(t.clone())),
            None => return Err(CmdError::MissingStatement),
        };
        Ok(statement)
    }

    fn parse_add(&mut self) -> Result<Statement> {
        match_token(&self.iter.next(), Token::Command(Command::Add))?;
        let datatype = self.parse_datatype()?;
        let key = self.parse_key()?;
        
        let value = match datatype {
            ValueType::Array(_) => self.parse_array()?,
            _ => self.parse_value()?
        };

        Ok(Statement::Add { key, value, datatype })
    }

    fn parse_datatype(&mut self) -> Result<ValueType> {
        let datatype = match self.iter.peek() {
            Some(Token::DataType(DataType::Null)) => ValueType::Null,
            Some(Token::DataType(DataType::Bool)) => ValueType::Bool,
            Some(Token::DataType(DataType::Int32)) => ValueType::Int32,
            Some(Token::DataType(DataType::Int64)) => ValueType::Int64,
            Some(Token::DataType(DataType::Float32)) => ValueType::Float32,
            Some(Token::DataType(DataType::Float64)) => ValueType::Float64,
            Some(Token::DataType(DataType::Char)) => ValueType::Char,
            Some(Token::DataType(DataType::String)) => ValueType::String,
            Some(Token::DataType(DataType::Array)) => {
                self.iter.next();
                match_token(&self.iter.next(), Token::Symbol(Symbol::LeftParen))?;
                let include_type = Box::new(self.parse_datatype()?);
                match_token(&self.iter.next(), Token::Symbol(Symbol::RightParen))?;
                return Ok(ValueType::Array(include_type));
            },
            _ => return Ok(ValueType::String),
        };
        self.iter.next();

        Ok(datatype)
    }

    fn parse_typeof(&mut self) -> Result<Statement> {
        match_token(&self.iter.next(), Token::Command(Command::TypeOf))?;
        let key = self.parse_key()?;
        Ok(Statement::TypeOf { key })
    }

    fn parse_get(&mut self) -> Result<Statement> {
        match_token(&self.iter.next(), Token::Command(Command::Get))?;
        let key = self.parse_key()?;
        Ok(Statement::Get { key })
    }

    fn parse_delete(&mut self) -> Result<Statement> {
        match_token(&self.iter.next(), Token::Command(Command::Delete))?;
        let key = self.parse_key()?;
        Ok(Statement::Delete { key })
    }

    fn parse_key(&mut self) -> Result<String> {
        if let Ok(ValueP::Identifier(s)) = self.parse_value() {
            return Ok(s);
        }
        Err(CmdError::MissingKey)
    }

    fn parse_user(&mut self) -> Result<Statement> {
        match_token(&self.iter.next(), Token::Command(Command::User))?;
        let cmd = match self.iter.peek() {
            Some(Token::Command(Command::Create)) => {
                self.iter.next();
                let user_info = self.parse_str_args()?;
                if user_info.len() != 3 {
                    return Err(CmdError::ParameterError("create user".to_string()));
                }
                UserCmd::Create {
                    info : UserInfo {
                        name: user_info[0].clone(),
                        password: user_info[1].clone(),
                        level: user_info[2].clone()
                    }
                }
            },
            Some(Token::Command(Command::Delete)) => {
                self.iter.next();
                if let ValueP::Identifier(s) = self.parse_value()? {
                    UserCmd::Delete { name: s };
                }
                return Err(CmdError::MissingArg);
            }
            Some(t) => return Err(CmdError::UnexpectedToken(t.clone())),
            None => return Err(CmdError::MissingSubCmd),
        };

        Ok(Statement::User{ cmd })
    }

    fn parse_list(&mut self) -> Result<Statement> {
        match_token(&self.iter.next(), Token::Command(Command::List))?;
        let arg = match self.iter.next() {
            Some(Token::Arg(Arg::Values)) => List::Values,
            Some(Token::Arg(Arg::Entries)) => List::Entries,
            Some(t) => return Err(CmdError::UnexpectedToken(t.clone())),
            None => return Err(CmdError::MissingArg),
        };
        Ok(Statement::List{list: arg})
    }

    fn parse_open(&mut self) -> Result<Statement> {
        match_token(&self.iter.next(), Token::Command(Command::Open))?;
        if let Some(ValueP::Identifier(s)) = self.iter.next().to_value() {
            return Ok(Statement::Open { 
                file: s
            });
        }
        Err(CmdError::MissingPath)
    }

    fn parse_str_args(&mut self) -> Result<Vec<String>> {
        match_token(&self.iter.next(), Token::Symbol(Symbol::LeftParen))?;
        let mut args: Vec<String> = Vec::new();
    
        loop {
            match self.iter.peek() {
                Some(Token::Symbol(Symbol::RightParen)) => break,
                Some(Token::Identifier(v)) => {
                    args.push(v.clone());
                    if let Some(Token::Symbol(Symbol::Comma)) = self.iter.peek() {
                        self.iter.next();
                    }
                },
                _ => return Err(CmdError::MissingValue),
            }
        }
        self.iter.next();
        Ok(args)
    }

    fn parse_value(&mut self) -> Result<ValueP> {
        let value = match self.iter.peek() {
            Some(Token::Identifier(s)) => ValueP::Identifier(s.clone()),
            Some(Token::Number(n)) => ValueP::Number(n.clone()),
            Some(Token::Bool(b)) => ValueP::Bool(*b),
            Some(Token::Symbol(Symbol::LeftParen)) => {
                self.iter.next();
                let next_value = self.parse_value()?;
                return match self.iter.next() {
                    Some(Token::Symbol(Symbol::RightParen)) => Ok(next_value),
                    _ => Err(CmdError::MissingToken(Token::Symbol(Symbol::RightParen))),
                };
            },
            Some(Token::Symbol(Symbol::LeftBracket)) => {
                self.iter.next();
                let next_value = self.parse_array()?;
                return Ok(next_value);
            }
            _ => return Err(CmdError::MissingValue),
        };
        self.iter.next();
        Ok(value)
    }

    fn parse_array(&mut self) -> Result<ValueP> {
        match_token(&self.iter.next(), Token::Symbol(Symbol::LeftBracket))?;
        let mut array = Vec::new();

        loop {
            match self.iter.peek() {
                Some(Token::Symbol(Symbol::RightBracket)) => break,
                Some(_) => (),
                None => return Err(CmdError::MissingValue),
            }
            
            if let Ok(v) = self.parse_value() {
                array.push(v);
                if let Some(Token::Symbol(Symbol::Comma)) = self.iter.peek() {
                    self.iter.next();
                }
            }
        }

        self.iter.next();
        Ok(ValueP::Array(Box::new(array)))
    }
}

fn match_token(value: &Option<Token>, expect: Token) -> Result<()> {
    return match value {
        Some(_) => Ok(()),
        None => return Err(CmdError::MissingToken(expect))
    }
}
