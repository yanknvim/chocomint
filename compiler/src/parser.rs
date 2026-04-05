use crate::tokenizer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Eq)]
pub enum Tree {
    BinOp(Op, Box<Tree>, Box<Tree>),
    Integer(isize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Tree {
        self.expr()
    }

    fn expr(&mut self) -> Tree {
        let mut tree = self.term();
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Plus => {
                    self.tokens.next();
                    let rhs = self.term();
                    tree = Tree::BinOp(Op::Add, Box::new(tree), Box::new(rhs));
                }
                Token::Minus => {
                    self.tokens.next();
                    let rhs = self.term();
                    tree = Tree::BinOp(Op::Sub, Box::new(tree), Box::new(rhs));
                }
                _ => break,
            }
        }

        tree
    }

    fn term(&mut self) -> Tree {
        let mut tree = self.unary();
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Asterisk => {
                    self.tokens.next();
                    let rhs = self.unary();
                    tree = Tree::BinOp(Op::Mul, Box::new(tree), Box::new(rhs));
                }
                Token::Slash => {
                    self.tokens.next();
                    let rhs = self.unary();
                    tree = Tree::BinOp(Op::Div, Box::new(tree), Box::new(rhs));
                }
                _ => break,
            }
        }

        tree
    }

    fn unary(&mut self) -> Tree {
        match self.tokens.peek() {
            Some(Token::Plus) => {
                self.tokens.next();
                self.primary()
            }
            Some(Token::Minus) => {
                self.tokens.next();
                let prim = self.primary();
                Tree::BinOp(Op::Sub, Box::new(Tree::Integer(0)), Box::new(prim))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Tree {
        match self.tokens.next() {
            Some(Token::Num(n)) => Tree::Integer(n as isize),
            Some(Token::LParen) => {
                let tree = self.expr();
                if self.tokens.next() != Some(Token::RParen) {
                    panic!("Expected )");
                }
                tree
            }
            _ => panic!("unexpected token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_precedence() {
        let mut parser = Parser::new(tokenize("1 + 2 * 3"));
        let tree = parser.parse();
        assert_eq!(
            tree,
            Tree::BinOp(
                Op::Add,
                Box::new(Tree::Integer(1)),
                Box::new(Tree::BinOp(
                    Op::Mul,
                    Box::new(Tree::Integer(2)),
                    Box::new(Tree::Integer(3))
                ))
            )
        );
    }

    #[test]
    fn parse_left_associative_sub() {
        let mut parser = Parser::new(tokenize("1 - 2 - 3"));
        let tree = parser.parse();
        assert_eq!(
            tree,
            Tree::BinOp(
                Op::Sub,
                Box::new(Tree::BinOp(
                    Op::Sub,
                    Box::new(Tree::Integer(1)),
                    Box::new(Tree::Integer(2))
                )),
                Box::new(Tree::Integer(3))
            )
        );
    }

    #[test]
    fn parse_unary_minus_paren() {
        let mut parser = Parser::new(tokenize("-(1 + 2)"));
        let tree = parser.parse();
        assert_eq!(
            tree,
            Tree::BinOp(
                Op::Sub,
                Box::new(Tree::Integer(0)),
                Box::new(Tree::BinOp(
                    Op::Add,
                    Box::new(Tree::Integer(1)),
                    Box::new(Tree::Integer(2))
                ))
            )
        );
    }

    #[test]
    fn parse_paren_precedence() {
        let mut parser = Parser::new(tokenize("(1 + 2) * 3"));
        let tree = parser.parse();
        assert_eq!(
            tree,
            Tree::BinOp(
                Op::Mul,
                Box::new(Tree::BinOp(
                    Op::Add,
                    Box::new(Tree::Integer(1)),
                    Box::new(Tree::Integer(2))
                )),
                Box::new(Tree::Integer(3))
            )
        );
    }

    #[test]
    #[should_panic(expected = "Expected )")]
    fn parse_missing_rparen_panics() {
        let mut parser = Parser::new(tokenize("(1 + 2"));
        parser.parse();
    }
}
