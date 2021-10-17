use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::lexer::token::Token;
use crate::lexer::token::TokenType::{FLOAT, INTEGER};

#[path = "./token.rs"]
mod token;

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
            if token.token_type != token::TokenType::NONE {
                tokens.push(token);
            }
        }
        return tokens;
    }

    fn advance(&mut self) {
        self.index += 1;
        self.current_char = if self.index < self.chars.len() { self.chars[self.index] } else { '\0' };
    }

    fn make_token(&mut self) -> Result<Token, IllegalCharError> {
        let token: Result<Token, IllegalCharError>;
        if [' ', '\n'].contains(&self.current_char) {
            token = Ok(Token {
                token_type: token::TokenType::NONE,
                value: String::new(),
            });
            self.advance();
        } else if ['+', '-', '*', '/', '(', ')'].contains(&self.current_char) {
            token = Ok(Token {
                token_type: token::TokenType::AS,
                value: self.current_char.to_string(),
            });
            self.advance();
        } else if self.current_char.is_numeric() {
            token = self.make_number()
        } else {
            token = Err(IllegalCharError);
        }
        return token;
    }

    fn make_number(&mut self) -> Result<Token, IllegalCharError> {
        let mut digits = String::new();
        let mut dot_count = 0;
        while self.current_char.is_numeric() || self.current_char == '.' {
            if self.current_char == '.' {
                if dot_count > 0 {
                    return Err(IllegalCharError);
                }
                dot_count += 1;
            }
            digits.push(self.current_char);

            self.advance();
        }

        return if dot_count == 0 {
            Ok(Token {
                token_type: INTEGER,
                value: digits,
            })
        } else {
            Ok(Token {
                token_type: FLOAT,
                value: digits,
            })
        };
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