use std::fmt::Debug;

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LeftParenthesis,
    RightParenthesis,
    Power
}

pub fn lex(text: &str) -> Result<Vec<Token>, String> {
    let mut chars = text.chars();
    let mut tokens: Vec<Token> = vec![];

    let mut option_c = chars.next();

    while let Some(c) = option_c {
        if c.is_ascii_whitespace() {
            option_c = chars.next();
            continue;
        }
        if c.is_ascii_digit() {
            let mut has_dot = false;
            let mut num_str = c.to_string();
            option_c = chars.next();
            while let Some(c) = option_c {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    option_c = chars.next();
                } else if c == '.' {
                    if has_dot {
                        break;
                    }
                    num_str.push('.');
                    has_dot = true;
                    option_c = chars.next();
                } else {
                    break;
                }
            }

            tokens.push(Token::Number(
                num_str
                    .parse()
                    .map_err(|e| format!("Error parsing number: {}", e))?,
            ));
            continue;
        }
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '%' => tokens.push(Token::Modulo),
            '(' => tokens.push(Token::LeftParenthesis),
            ')' => tokens.push(Token::RightParenthesis),
            '^' => tokens.push(Token::Power),
            _ => return Err(format!("Invalid character: {}", c)),
        }
        option_c = chars.next();
    }

    Ok(tokens)
}
