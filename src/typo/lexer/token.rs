use std::fmt;
use std::fmt::{Formatter, Pointer};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    AS,
    FLOAT,
    INT,
    NONE
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}", self)
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[Type: {}, Value: {}]", self.token_type, self.value)
    }
}