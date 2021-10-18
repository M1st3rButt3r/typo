use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum Token {
    AS(char),
    FLOAT(f32),
    INTEGER(i32),
    NONE
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}", self)
    }
}