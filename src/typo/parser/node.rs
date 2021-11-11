use std::fmt;
use std::fmt::Formatter;
use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    None,
    Number(Token),
    BinOp(Token, Box<Node>, Box<Node>),
    UnOp(Token, Box<Node>)
}

impl Node {
    pub fn visit(&self) -> Result<f32, RuntimeError> {
        return match self {
            Node::None => Err(RuntimeError),
            Node::Number(token) => Node::visit_number_node(*token),
            Node::BinOp(_, _, _) => Err(RuntimeError),
            Node::UnOp(_, _) => Err(RuntimeError),

        }
    }

    pub fn visit_number_node(tkn: Token) -> Result<f32, RuntimeError> {
        if let Token::FLOAT(value) = tkn {
            return Ok(value)
        }
        return Err(RuntimeError)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}", self)
    }
}

#[derive(Debug)]
pub struct RuntimeError;

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime Error")
    }
}

impl std::error::Error for RuntimeError {}