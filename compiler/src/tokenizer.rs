#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Num(usize),

    Plus,
    Minus,
    Asterisk,
    Slash,

    Eq,
    NotEq,
    GreaterThan,
    LessThan,
    GreaterThanOrEq,
    LessThanOrEq,

    LParen,
    RParen,
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut s = s.chars().peekable();
    while let Some(c) = s.next() {
        match c {
            c if c.is_ascii_digit() => {
                let mut number = String::from(c);
                while let Some(&c) = s.peek()
                    && c.is_ascii_digit()
                {
                    s.next();
                    number.push(c);
                }

                tokens.push(Token::Num(number.parse().unwrap()));
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::Slash),
            '=' => {
                if let Some(&c) = s.peek() && c == '=' {
                    s.next();
                    tokens.push(Token::Eq);
                } else {
                    unimplemented!();
                }
            }
            '!' => {
                if let Some(&c) = s.peek() && c == '=' {
                    s.next();
                    tokens.push(Token::NotEq);
                } else {
                    unimplemented!();
                }
            }
            '>' => {
                if let Some(&c) = s.peek() && c == '=' {
                    s.next();
                    tokens.push(Token::GreaterThanOrEq);
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            '<' => {
                if let Some(&c) = s.peek() && c == '=' {
                    s.next();
                    tokens.push(Token::LessThanOrEq);
                } else {
                    tokens.push(Token::LessThan);
                }
            }
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

    #[test]
    fn test_eq() {
        let tokens = tokenize("1 == 1");
        assert_eq!(
            tokens,
            vec![
                Token::Num(1),
                Token::Eq,
                Token::Num(1),
            ]
        );

        let tokens = tokenize("2 != 1");
        assert_eq!(
            tokens,
            vec![
                Token::Num(2),
                Token::NotEq,
                Token::Num(1),
            ]
        );
    }

    #[test]
    fn test_relational() {
        let tokens = tokenize("2 > 1");
        assert_eq!(
            tokens,
            vec![
                Token::Num(2),
                Token::GreaterThan,
                Token::Num(1),
            ]
        );

        let tokens = tokenize("1 < 2");
        assert_eq!(
            tokens,
            vec![
                Token::Num(1),
                Token::LessThan,
                Token::Num(2),
            ]
        );

        let tokens = tokenize("2 >= 1");
        assert_eq!(
            tokens,
            vec![
                Token::Num(2),
                Token::GreaterThanOrEq,
                Token::Num(1),
            ]
        );

        let tokens = tokenize("1 <= 2");
        assert_eq!(
            tokens,
            vec![
                Token::Num(1),
                Token::LessThanOrEq,
                Token::Num(2),
            ]
        );
    }
}
