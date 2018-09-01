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

    fn pop_buffer_to_tokens(buffer: &mut Vec<char>, tokens: &mut Vec<Token>, kind: String) {
        tokens.push(
            Token { kind: kind,
                    value: buffer.clone() }
        );
        buffer.clear();
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer: Vec<char> = Vec::new();

    for character in expression.chars() {
        match character {
            c if is_digit(c) => {
                buffer.push(character);
            },
            c if is_operator(c) => {
                if !(buffer.is_empty()) {
                    let kind = String::from("literal");
                    pop_buffer_to_tokens(&mut buffer, &mut tokens, kind);
                }

                buffer.push(character);
                let kind = String::from("operator");
                pop_buffer_to_tokens(&mut buffer, &mut tokens, kind);
            },
            _ => ()
        };
    }

    // A valid expression will never end with an operator
    let kind = String::from("literal");
    pop_buffer_to_tokens(&mut buffer, &mut tokens, kind);

    return tokens;
}
