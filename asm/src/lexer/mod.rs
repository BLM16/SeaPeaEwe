mod token;

pub use token::Token;
use crate::ir::Register;

pub fn lex(mut line: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::with_capacity(8);

    while !line.is_empty() {
        line = line.trim_start();
        if line.is_empty() {
            break;
        }

        let (tok, rest) = match line.as_bytes()[0] {
            b'+' => (Token::Plus, &line[1..]),
            b'-' => (Token::Minus, &line[1..]),
            b':' => (Token::Colon, &line[1..]),
            b',' => (Token::Comma, &line[1..]),

            b'0'..=b'9' => {
                let end = line.find(|c: char| !c.is_ascii_digit()).unwrap_or(line.len());
                let val = line[..end].parse().unwrap();
                (Token::Number(val), &line[end..])
            }

            _ => {
                let end = line.find(|c: char| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(line.len());
                let ident = &line[..end];

                if let Some(r) = Register::parse(ident) {
                    (Token::Register(r), &line[end..])
                } else {
                    (Token::Identifier(ident), &line[end..])
                }
            }
        };

        tokens.push(tok);
        line = rest;
    }

    tokens
}
