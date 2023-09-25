use super::{
    token::{
        token::*,
        symbol::SymbolExtChar
    },
};

pub fn lex(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(&token) = chars.peek() {
        match token {
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
            }
            '\'' | '"' => {
                if let Some(quote) = chars.next() {
                    let literal = collect_until(&mut chars, |c| c == quote);
                    tokens.push(Token::Identifier(literal));
                    chars.next();
                }
            }
            token if token.is_ascii_digit() => {
                let num = collect_until(&mut chars, |c| !c.is_ascii_digit() && c != '.');
                tokens.push(Token::Number(num));
            }
            token if token.is_symbol() => {
                let mut symbol = token.to_string();

                if token.has_next(&mut chars) {
                    symbol.push(chars.next().take().unwrap());
                } else {
                    chars.next();
                }

                if let Some(s) = symbol.as_symbol() {
                    tokens.push(Token::Symbol(s));
                }
            }
            _ => {
                let text = collect_until(&mut chars, |c| !c.is_alphanumeric() && c != '_' );
                if let Some(command) = text.as_command() {
                    tokens.push(Token::Command(command));
                } else if let Some(datatype) = text.as_datatype() {
                    tokens.push(Token::DataType(datatype));
                } else if let Some(arg) = text.as_arg() {
                    tokens.push(Token::Arg(arg));
                } else if let Some(bool) = text.as_bool() {
                    tokens.push(Token::Bool(bool));
                } else {
                    tokens.push(Token::Identifier(text));
                }
            }
        }
    }
    tokens
}

fn collect_until<F>(chars: &mut std::iter::Peekable<std::str::Chars>, condition: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut result = String::new();

    while let Some(&c) = chars.peek() {
        if condition(c) {
            break;
        }
        result.push(c);
        chars.next();
    }
    result
}
