use crate::lexer::{IllegalCharError, Lexer};
use crate::lexer::token::Token;


pub static RULES: &'static [Rule] = &[
    Rule {
        evaluate: (|char| [' ', '\n'].contains(&char)),
        create: (|lexer| {
            let token = Ok(Token::NONE);
            lexer.advance();
            return token;
        }),
    },
    Rule {
        evaluate: (|char| ['+', '-', '*', '/', '(', ')'].contains(&char)),
        create: (|lexer| {
            let token = Ok(Token::AS(lexer.current_char));
            lexer.advance();
            return token;
        }),
    },
    Rule {
        evaluate: (|char| char.is_numeric() || char == '.'),
        create: (|lexer| {
            let mut digits = String::new();
            let mut dot_count = 0;
            while lexer.current_char.is_numeric() || lexer.current_char == '.' {
                if lexer.current_char == '.' {
                    if dot_count > 0 {
                        return Err(IllegalCharError);
                    }
                    dot_count += 1;
                }
                digits.push(lexer.current_char);

                lexer.advance();
            }

            return if dot_count == 0 {
                Ok(Token::INTEGER(digits.parse().unwrap()))
            } else {
                Ok(Token::FLOAT(digits.parse().unwrap()))
            };
        }),
    }
];

pub struct Rule {
    pub evaluate: fn(char) -> bool,
    pub create: fn(&mut Lexer) -> Result<Token, IllegalCharError>,
}