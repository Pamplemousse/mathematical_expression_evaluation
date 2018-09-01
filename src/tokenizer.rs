use std::fmt::{Debug, Display, Formatter, Error};

pub struct Token {
    kind: String,
    value: String
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}: {}", self.kind, self.value)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Token {{ kind: {}, value: {} }}", self.kind, self.value)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.kind == other.kind
          && self.value == other.value
    }
}

pub fn tokenize(expression: &str) -> Vec<Token> {
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
                    value: buffer.clone().into_iter().collect() }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_creates_tokens_from_a_string() {
        let string = "2*3";
        let result: Vec<Token> = tokenize(string);

        let mut expected_result: Vec<Token> = Vec::new();
        expected_result.push(Token { kind: String::from("literal"), value: String::from("2") });
        expected_result.push(Token { kind: String::from("operator"), value: String::from("*") });
        expected_result.push(Token { kind: String::from("literal"), value: String::from("3") });

        assert_eq!(result, expected_result);
    }

    #[test]
    fn tokenize_deals_with_multiple_digits_numbers() {
        let string = "22*33";
        let result: Vec<Token> = tokenize(string);

        let mut expected_result: Vec<Token> = Vec::new();
        expected_result.push(Token { kind: String::from("literal"), value: String::from("22") });
        expected_result.push(Token { kind: String::from("operator"), value: String::from("*") });
        expected_result.push(Token { kind: String::from("literal"), value: String::from("33") });

        assert_eq!(result, expected_result);
    }
}
