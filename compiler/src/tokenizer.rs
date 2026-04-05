use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Num(usize),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut s_iter = s.chars().peekable();
    while let Some(c) = s_iter.next() {
        match c {
            c if c.is_ascii_digit() => {
                let mut number = String::from(c);
                while let Some(&c) = s_iter.peek()
                    && c.is_ascii_digit()
                {
                    s_iter.next();
                    number.push(c);
                }

                tokens.push(Token::Num(number.parse().unwrap()));
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            c if c.is_whitespace() => {}
            _ => panic!("unexpected character: {}", c),
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("1 \n * 2 +    3");
        assert_eq!(
            tokens,
            vec![
                Token::Num(1),
                Token::Asterisk,
                Token::Num(2),
                Token::Plus,
                Token::Num(3)
            ]
        );
    }

    #[test]
    fn test_paren() {
        let tokens = tokenize("(1 + 2) * 3");
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Num(1),
                Token::Plus,
                Token::Num(2),
                Token::RParen,
                Token::Asterisk,
                Token::Num(3)
            ]
        );
    }
}
