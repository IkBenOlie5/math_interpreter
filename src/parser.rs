use crate::lexer::Token;
use std::fmt::Debug;
use std::slice::Iter;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug)]
pub enum Node {
    Number(f64),
    Unary {
        op: Operator,
        node: Box<Node>,
    },
    Binary {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
}

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    cur_token: Option<&'a Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        let mut tokens = tokens.iter();
        let cur_token = tokens.next();
        Parser { tokens, cur_token }
    }

    pub fn parse(&mut self) -> Result<Option<Node>, String> {
        if self.cur_token.is_none() {
            return Ok(None);
        }

        let result = self.expr()?;
        if self.cur_token.is_some() {
            Err("Invalid syntax".to_string())
        } else {
            Ok(Some(result))
        }
    }

    fn expr(&mut self) -> Result<Node, String> {
        let mut result = self.term()?;
        while let Some(t) = self.cur_token && matches!(t, Token::Plus | Token::Minus) {
            match t {
                Token::Plus => {
                    self.cur_token = self.tokens.next();
                    result = Node::Binary{op: Operator::Plus, left: Box::new(result), right: Box::new(self.term()?)};
                },
                Token::Minus => {
                    self.cur_token = self.tokens.next();
                    result = Node::Binary{op: Operator::Minus, left: Box::new(result), right: Box::new(self.term()?)};
                    },
                _ => unreachable!(),
            }
        }
        Ok(result)
    }

    fn term(&mut self) -> Result<Node, String> {
        let mut result = self.factor()?;
        while let Some(t) = self.cur_token && matches!(t, Token::Multiply | Token::Divide | Token::Modulo) {
            match t {
                Token::Multiply => {
                    self.cur_token = self.tokens.next();
                    result = Node::Binary{op: Operator::Multiply, left: Box::new(result), right: Box::new(self.factor()?)};
                },
                Token::Divide => {
                    self.cur_token = self.tokens.next();
                    result = Node::Binary{op: Operator::Divide, left: Box::new(result), right: Box::new(self.factor()?)};
                },
                Token::Modulo => {
                    self.cur_token = self.tokens.next();
                    result = Node::Binary{op: Operator::Modulo, left: Box::new(result), right: Box::new(self.factor()?)};
                },
                _ => unreachable!(),
            }
        }
        Ok(result)
    }

    fn factor(&mut self) -> Result<Node, String> {
        if let Some(t) = self.cur_token {
            match t {
                Token::Number(n) => {
                    self.cur_token = self.tokens.next();
                    Ok(Node::Number(*n))
                }
                Token::Plus => {
                    self.cur_token = self.tokens.next();
                    Ok(Node::Unary {
                        op: Operator::Plus,
                        node: Box::new(self.factor()?),
                    })
                }
                Token::Minus => {
                    self.cur_token = self.tokens.next();
                    Ok(Node::Unary {
                        op: Operator::Minus,
                        node: Box::new(self.factor()?),
                    })
                }
                _ => Err("Invalid syntax".to_string()),
            }
        } else {
            Err("Invalid syntax".to_string())
        }
    }
}
