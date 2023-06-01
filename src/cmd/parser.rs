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
            Some(Token::Command(Command::User)) => self.parse_user()?,
            Some(Token::Command(Command::Compact)) => Statement::Compact,
            Some(Token::Command(Command::Quit)) => Statement::Quit,
            Some(t) => return Err(CmdError::UnexpectedToken(t.clone())),
            None => return Err(CmdError::MissingStatement),
        };
        Ok(statement)
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
                if let Value::Identifier(s) = self.parse_value()? {
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
        if let Some(Value::Identifier(s)) = self.iter.next().to_value() {
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

    fn parse_args(&mut self) -> Result<Vec<Value>> {
        match_token(&self.iter.next(), Token::Symbol(Symbol::LeftParen))?;
        let mut args: Vec<Value> = Vec::new();
    
        loop {
            match self.iter.peek() {
                Some(Token::Symbol(Symbol::RightParen)) => break,
                Some(t) => (),
                _ => return Err(CmdError::MissingValue),
            }
            if let Ok(v) = self.parse_value() {
                args.push(v);
                if let Some(Token::Symbol(Symbol::Comma)) = self.iter.peek() {
                    self.iter.next();
                }
            }
        }
        self.iter.next();
        Ok(args)
    }

    fn parse_value(&mut self) -> Result<Value> {
        let value = match self.iter.peek() {
            Some(Token::Identifier(s)) => Value::Identifier(s.clone()),
            Some(Token::Number(n)) => Value::Number(n.clone()),
            Some(Token::Bool(b)) => Value::Bool(*b),
            _ => return Err(CmdError::MissingValue),
        };
        self.iter.next();
        Ok(value)
    }
}

fn match_token(value: &Option<Token>, expect: Token) -> Result<()> {
    return match value {
        Some(_) => Ok(()),
        None => return Err(CmdError::MissingToken(expect))
    }
}
