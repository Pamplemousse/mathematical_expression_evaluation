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
        let token: Option<Token> = match character {
            c if is_digit(c) => {
                let kind = String::from("literal");
                let mut characters: Vec<char> = Vec::new();
                characters.push(c);
                Some(Token { kind: kind, value: characters })
            },
            c if is_operator(c) => {
                let kind = String::from("operator");
                let mut characters: Vec<char> = Vec::new();
                characters.push(c);
                Some(Token { kind: kind, value: characters })
            },
            _ => None
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
