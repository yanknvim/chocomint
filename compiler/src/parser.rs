use crate::tokenizer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree {
    BinOp(Op, Box<Tree>, Box<Tree>),
    Integer(isize),
    Var(String),
    Assign(Box<Tree>, Box<Tree>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,

    Eq,
    NotEq,

    GreaterThan,
    LessThan,
    GreaterThanOrEq,
    LessThanOrEq,
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

    pub fn parse(&mut self) -> Vec<Tree> {
        let mut trees = Vec::new();
        while let Some(_) = self.tokens.peek() {
            let stmt = self.statement();
            trees.push(stmt);
        }

        trees
    }

    fn statement(&mut self) -> Tree {
        let tree = self.expr();

        match self.tokens.peek() {
            Some(Token::Semicolon) => {
                self.tokens.next();
            }
            other => panic!("Unexpected Token: Expected ; but found {:?}", other),
        }

        tree
    }

    fn expr(&mut self) -> Tree {
        self.assign()
    }

    fn assign(&mut self) -> Tree {
        let lhs = self.equality();
        if let Some(Token::Assign) = self.tokens.peek() {
            self.tokens.next();
            let rhs = self.assign();
            Tree::Assign(Box::new(lhs), Box::new(rhs))
        } else {
            return lhs;
        }
    }

    fn equality(&mut self) -> Tree {
        let mut tree = self.relational();
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Eq => {
                    self.tokens.next();
                    let rhs = self.relational();
                    tree = Tree::BinOp(Op::Eq, Box::new(tree), Box::new(rhs));
                }
                Token::NotEq => {
                    self.tokens.next();
                    let rhs = self.relational();
                    tree = Tree::BinOp(Op::NotEq, Box::new(tree), Box::new(rhs));
                }
                _ => break,
            }
        }

        tree
    }

    fn relational(&mut self) -> Tree {
        let mut tree = self.add();
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::GreaterThan => {
                    self.tokens.next();
                    let rhs = self.add();
                    tree = Tree::BinOp(Op::GreaterThan, Box::new(tree), Box::new(rhs));
                }
                Token::GreaterThanOrEq => {
                    self.tokens.next();
                    let rhs = self.add();
                    tree = Tree::BinOp(Op::GreaterThanOrEq, Box::new(tree), Box::new(rhs));
                }
                Token::LessThan => {
                    self.tokens.next();
                    let rhs = self.add();
                    tree = Tree::BinOp(Op::LessThan, Box::new(tree), Box::new(rhs));
                }
                Token::LessThanOrEq => {
                    self.tokens.next();
                    let rhs = self.add();
                    tree = Tree::BinOp(Op::LessThanOrEq, Box::new(tree), Box::new(rhs));
                }
                _ => break,
            }
        }

        tree
    }

    fn add(&mut self) -> Tree {
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
            Some(Token::Ident(s)) => Tree::Var(s),
            Some(Token::Num(n)) => Tree::Integer(n as isize),
            Some(Token::LParen) => {
                let tree = self.expr();
                if self.tokens.next() != Some(Token::RParen) {
                    panic!("Expected )");
                }
                tree
            }
            other => panic!("unexpected token: {:?}", other),
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
        let tree = parser.expr();
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
        let tree = parser.expr();
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
        let tree = parser.expr();
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
        let tree = parser.expr();
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
    fn parse_relational_precedence() {
        let mut parser = Parser::new(tokenize("1 + 2 > 3"));
        let tree = parser.expr();
        assert_eq!(
            tree,
            Tree::BinOp(
                Op::GreaterThan,
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
    fn parse_equality_precedence() {
        let mut parser = Parser::new(tokenize("1 + 2 == 3"));
        let tree = parser.expr();
        assert_eq!(
            tree,
            Tree::BinOp(
                Op::Eq,
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
        parser.expr();
    }
}
