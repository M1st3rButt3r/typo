use std::io;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::io::{Error, Write};
use crate::lexer::token::Token;

#[path = "./token.rs"] mod token;

pub struct Lexer {
    pub chars: Vec<char>,
    pub current_char: char,
    pub index: usize
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        let mut chars: Vec<char> = code.chars().collect();

        return Lexer {
            current_char: chars[0],
            chars,
            index: 0
        }
    }

    fn advance(&mut self) {
        self.index += 1;
        self.current_char = if self.index < self.chars.len() {self.chars[self.index]}  else {'\0'};

    }

    pub fn make_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.current_char != '\0' {
            let mut token = self.token().ok().unwrap();
            if token.token_type != token::TokenType::NONE {
                tokens.push(token);
            }

            self.advance()
        }
        return tokens
    }

    fn token(&mut self) -> Result<Token, IllegalCharError> {
        return if [' ', '\n'].contains(&self.current_char) {
            self.advance();
            return Ok(Token {
                token_type: token::TokenType::NONE,
                value: String::new()
            })
        } else if ['+', '-', '*', '/', '(', ')'].contains(&self.current_char) {
            Ok(Token {
                token_type: token::TokenType::AS,
                value: self.current_char.to_string()
            })
        }
        else {
            Err(IllegalCharError)
        }
    }
}

#[derive(Debug)]
struct IllegalCharError;

impl fmt::Display for IllegalCharError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Illegal Char")
    }
}

impl std::error::Error for IllegalCharError {}