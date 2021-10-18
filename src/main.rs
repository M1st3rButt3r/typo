use std::io;
use std::io::Write;
use crate::lexer::Lexer;

#[path = "typo/lexer/lexer.rs"] mod lexer;

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

        for token in lexer.make_tokens() {
            println!("{}", token)
        }
    }
}