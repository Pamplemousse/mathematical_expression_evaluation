use std::fmt::{Display, Formatter, Error};

struct Token {
    kind: String,
    value: char
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}: {}", self.kind, self.value)
    }
}

fn main() {
    let string = "22*3+4";

    let tokens: Vec<Token> = tokenize(string);
    for token in tokens {
        println!("{}", token)
    }

    println!("\n{} = ...42", string);
}

fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for character in expression.chars() {
        let kind: Option<String> = match character {
            c if is_digit(c) => Some(String::from("literal")),
            c if is_operator(c) => Some(String::from("operator")),
            _ => None
        };

        let token: Option<Token> = match kind {
            Some(s) => Some(Token { kind: s, value: character }),
            None => None
        };

        match token {
            Some(x) => tokens.push(x),
            None => ()
        }
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
