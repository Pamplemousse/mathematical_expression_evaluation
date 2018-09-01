mod tokenizer;

use tokenizer::*;

fn main() {
    let string = "22*35+444";

    let tokens: Vec<Token> = tokenize(string);
    for token in tokens {
        println!("{}", token)
    }

    println!("\n{} = ...42", string);
}
