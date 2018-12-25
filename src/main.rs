use std::env;

mod parser;
mod tokenizer;

use parser::parse;
use tokenizer::tokenize;
use tokenizer::token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let string = &args[1];

    let tokens: Vec<Token> = tokenize(string);
    let ast: Vec<Token> = parse(tokens);
    println!("{:?}", ast);
}
