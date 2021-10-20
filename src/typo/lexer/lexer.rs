use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::lexer::token::Token;

#[path = "rules.rs"]
mod rules;

#[path = "token.rs"]
pub mod token;

pub struct Lexer {
    pub chars: Vec<char>,
    pub current_char: char,
    pub index: usize,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        let chars: Vec<char> = code.chars().collect();
        return Lexer {
            current_char: chars[0],
            chars,
            index: 0,
        };
    }

    pub fn make_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.current_char != '\0' {
            let token = self.make_token().ok().unwrap();
            if token != token::Token::NONE {
                tokens.push(token);
            }
        }
        return tokens;
    }

    fn make_token(&mut self) -> Result<Token, IllegalCharError> {
        for rule in rules::RULES {
            if (rule.evaluate)(self.current_char) {
                return (rule.create)(self);
            }
        }
        return Err(IllegalCharError);
    }

    pub fn advance(&mut self) {
        self.index += 1;
        self.current_char = if self.index < self.chars.len() { self.chars[self.index] } else { '\0' };
    }
}

#[derive(Debug)]
pub struct IllegalCharError;

impl fmt::Display for IllegalCharError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Illegal Char")
    }
}

impl std::error::Error for IllegalCharError {}