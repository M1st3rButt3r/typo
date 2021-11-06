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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}", self)
    }
}