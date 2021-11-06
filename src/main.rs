use std::io;
use std::io::Write;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[path = "typo/lexer/lexer.rs"] mod lexer;
#[path = "typo/parser/parser.rs"] mod parser;

fn main() {
    loop {
        let mut input = String::new();
        print!("typo> ");
        io::stdout().flush()
            .expect("[Shell Error]: Flush");

        io::stdin()
            .read_line(&mut input)
            .expect("[Shell Error]: Wrong input");

        let mut lexer = Lexer::new(input);

        let mut parser = Parser::new(&mut lexer.make_tokens());

        println!("{}", parser.parse().unwrap());
    }
}