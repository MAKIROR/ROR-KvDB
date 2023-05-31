use super::token::*;

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
                    let literal = collect_until(&mut chars, |c, _| c == quote);
                    tokens.push(Token::Identifier(literal));
                    chars.next();
                }
            }
            token if token.is_ascii_digit() => {
                let num = collect_until(&mut chars, |c, _| !c.is_ascii_digit() && c != '.');
                tokens.push(Token::Number(num));
            }
            _ => {
                let text = collect_until(&mut chars, |c, result| !c.is_alphanumeric() && c != '_' );
                if let Some(command) = text.as_command() {
                    tokens.push(Token::Command(command));
                } else if let Some(datatype) = text.as_datatype() {
                    tokens.push(Token::DataType(datatype));
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
    F: Fn(char, String) -> bool,
{
    let mut result = String::new();

    while let Some(&c) = chars.peek() {
        if condition(c, result.clone()) {
            break;
        }
        result.push(c);
        chars.next();
    }
    result
}