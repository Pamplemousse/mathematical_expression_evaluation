use std::fmt::{Display, Formatter, Error};

struct Token {
    kind: String,
    value: Vec<char>
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let value: String = self.value.iter().collect();
        write!(f, "{}: {}", self.kind, value)
    }
}

fn main() {
    let string = "22*35+444";

    let tokens: Vec<Token> = tokenize(string);
    for token in tokens {
        println!("{}", token)
    }

    println!("\n{} = ...42", string);
}

fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut literal_buffer: Vec<char> = Vec::new();

    for character in expression.chars() {
        match character {
            c if is_digit(c) => {
                literal_buffer.push(character);
            },
            c if is_operator(c) => {
                if !(literal_buffer.is_empty()) {
                    let kind = String::from("literal");
                    tokens.push(
                        Token { kind: kind, value: literal_buffer.clone() }
                    );
                    literal_buffer.clear();
                }

                let kind = String::from("operator");
                let mut characters: Vec<char> = Vec::new();
                characters.push(c);
                tokens.push(
                    Token { kind: kind, value: characters }
                );
            },
            _ => ()
        };
    }

    if !(literal_buffer.is_empty()) {
        let kind = String::from("literal");
        tokens.push(
            Token { kind: kind, value: literal_buffer.clone() }
        );
        literal_buffer.clear();
    }

    return tokens;
}

fn is_digit(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false
    }
}

fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '^' => true,
        _ => false
    }
}
