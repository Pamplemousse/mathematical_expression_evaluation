use std::env;

mod tokenizer;
use tokenizer::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let string = &args[1];

    let tokens: Vec<Token> = tokenize(string);
    for token in tokens {
        println!("{}", token)
    }
}
