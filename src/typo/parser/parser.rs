#[path = "./node.rs"] mod node;
use std::boxed::Box;
use crate::lexer::token::Token;
use std::fmt::{Debug, Formatter};
use std::vec::Vec;
use std::fmt;
use std::result::Result;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    index: usize,
}

impl Parser {
    pub fn new(tokens: &mut Vec<Token>) -> Parser {
        let unmut_tokens = tokens.to_vec();
        Parser {
            tokens: unmut_tokens,
            current_token: tokens[0],
            index: 0
        }
    }

    pub fn parse(&mut self) -> Result<node::Node, SyntaxError> {
        let res = self.expr();
        if self.current_token != Token::NONE {
            return Err(SyntaxError);
        }
        return res;
    }

    fn advance(&mut self) {
        self.index += 1;
        self.current_token = if self.tokens.len() > self.index {self.tokens[self.index]} else {Token::NONE};
    }

    fn factor(&mut self) -> Result<node::Node, SyntaxError> {
        if matches!(self.current_token, Token::AS('+')) || matches!(self.current_token, Token::AS('-')) {
            let ars = self.current_token;
            self.advance();
            return Ok(node::Node::UnOp(ars, Box::new(self.factor().unwrap())))
        } else if matches!(self.current_token, Token::AS('(')) {
            self.advance();
            let expr = self.expr();
            if matches!(self.current_token, Token::AS(')')) {
                self.advance();
                return Ok(expr.unwrap())
            }
        } else if matches!(self.current_token, Token::FLOAT(_)) || matches!(self.current_token, Token::INTEGER(_))  {
            let node = node::Node::Number(self.current_token);
            self.advance();
            return Ok(node);
        }
        return Err(SyntaxError);
    }

    fn term(&mut self) -> Result<node::Node, SyntaxError> {
        return self.binary_operation(|parser| return parser.factor(), [Box::new(Token::AS('*')), Box::new(Token::AS('/'))]);
    }

    fn expr(&mut self) -> Result<node::Node, SyntaxError> {
        return self.binary_operation(|parser| return parser.term(), [Box::new(Token::AS('+')), Box::new(Token::AS('-'))]);
    }

    fn binary_operation(&mut self, f: fn(&mut Parser) -> Result<node::Node, SyntaxError>, operators: [Box<Token>; 2]) -> Result<node::Node, SyntaxError> {
        let mut return_node = f(self);

        while operators.contains(&Box::new(self.current_token)) {
            let op_tok = self.current_token;
            self.advance();
            let right = f(self);
            return_node = Ok(node::Node::BinOp(op_tok, Box::new(return_node.unwrap()), Box::new(right.unwrap())));
        }

        return return_node;
    }
}

#[derive(Debug)]
pub struct SyntaxError;

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax Error")
    }
}

impl std::error::Error for SyntaxError {}