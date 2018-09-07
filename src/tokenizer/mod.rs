pub mod token;
use self::token::*;

pub fn tokenize(expression: &str) -> Vec<Token> {
    fn is_left_parenthesis(c: char) -> bool {
        c == '('
    }

    fn is_right_parenthesis(c: char) -> bool {
        c == ')'
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
        if is_digit(character) {
            buffer.push(character);
            continue;
        }

        if !(buffer.is_empty()) {
            let kind = String::from("literal");
            pop_buffer_to_tokens(&mut buffer, &mut tokens, kind);
        }

        buffer.push(character);

        let kind: Option<String> = match character {
            c if is_left_parenthesis(c) => {
                Some(String::from("left_parenthesis"))
            },
            c if is_right_parenthesis(c) => {
                Some(String::from("right_parenthesis"))
            },
            c if is_operator(c) => {
                Some(String::from("operator"))
            },
            _ => None
        };

        match kind {
            Some(kind) => pop_buffer_to_tokens(&mut buffer, &mut tokens, kind),
            _ => ()
        }
    }

    // If the expression finishes with a literal, the buffer will contain it
    if !(buffer.is_empty()) {
        let kind = String::from("literal");
        pop_buffer_to_tokens(&mut buffer, &mut tokens, kind);
    }

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
        [("literal", "2"), ("operator", "*"), ("literal", "3")]
            .iter()
            .map(|tuple| (String::from(tuple.0), String::from(tuple.1)))
            .map(|tuple| Token { kind: tuple.0, value: tuple.1 })
            .for_each(|token| expected_result.push(token));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn tokenize_deals_with_multiple_digits_numbers() {
        let string = "22*33";
        let result: Vec<Token> = tokenize(string);

        let mut expected_result: Vec<Token> = Vec::new();
        [("literal", "22"), ("operator", "*"), ("literal", "33")]
            .iter()
            .map(|tuple| (String::from(tuple.0), String::from(tuple.1)))
            .map(|tuple| Token { kind: tuple.0, value: tuple.1 })
            .for_each(|token| expected_result.push(token));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn tokenize_deals_with_parentheses() {
        let string = "2*(4-3)";
        let result: Vec<Token> = tokenize(string);

        let mut expected_result: Vec<Token> = Vec::new();

        [("literal", "2"), ("operator", "*"), ("left_parenthesis", "("),
         ("literal", "4"), ("operator", "-"), ("literal", "3"),
         ("right_parenthesis", ")")]
            .iter()
            .map(|tuple| (String::from(tuple.0), String::from(tuple.1)))
            .map(|tuple| Token { kind: tuple.0, value: tuple.1 })
            .for_each(|token| expected_result.push(token));

       assert_eq!(result, expected_result);
    }
}