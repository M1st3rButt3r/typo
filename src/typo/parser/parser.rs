#[path = "./node.rs"] mod node;
use std::boxed::Box;
use crate::lexer::token::Token;
use std::vec::Vec;

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

    pub fn parse(&mut self) -> node::Node {
        let res = self.expr();
        return res;
    }

    fn advance(&mut self) {
        self.index += 1;
        self.current_token = if self.tokens.len() > self.index {self.tokens[self.index]} else {Token::NONE};
    }

    fn factor(&mut self) -> node::Node {
        if matches!(self.current_token, Token::FLOAT(_)) || matches!(self.current_token, Token::INTEGER(_))  {
            let node = node::Node::Number(self.current_token);
            self.advance();
            return node;
        }
        return node::Node::None;
    }

    fn term(&mut self) -> node::Node {
        return self.binary_operation(|parser| return parser.factor(), [Box::new(Token::AS('*')), Box::new(Token::AS('/'))]);
    }

    fn expr(&mut self) -> node::Node {
        return self.binary_operation(|parser| return parser.term(), [Box::new(Token::AS('+')), Box::new(Token::AS('-'))]);
    }

    fn binary_operation(&mut self, f: fn(&mut Parser) -> node::Node, operators: [Box<Token>; 2]) -> node::Node {
        let mut return_node = f(self);
        while operators.contains(&Box::new(self.current_token)) {
            let op_tok = self.current_token;
            self.advance();
            let right = f(self);
            return_node = node::Node::BinOp(op_tok, Box::new(return_node), Box::new(right));
        }

        return return_node;
    }
}
