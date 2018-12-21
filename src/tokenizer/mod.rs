pub mod token;
use self::token::*;
use self::token::literal::Literal;
use self::token::literal::digit::Digit;

pub fn tokenize(expression: &str) -> Vec<Token> {
    fn pop_literal_buffer_to_tokens(buffer: &mut Vec<Digit>, tokens: &mut Vec<Token>) {
        let token: Token = Token::Literal(
            Literal::new(buffer.clone())
        );
        tokens.push(token);
        buffer.clear();
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut literal_buffer: Vec<Digit> = Vec::new();

    for character in expression.chars() {
        let digit: Option<Digit> = Digit::new(character);
        match digit {
            Some(digit) => {
                literal_buffer.push(digit);
                continue;
            },
            None => ()
        }

        if !(literal_buffer.is_empty()) {
            pop_literal_buffer_to_tokens(&mut literal_buffer, &mut tokens);
        }

        let token: Option<Token> = Token::from_char(character);

        match token {
            Some(token) => tokens.push(token),
            _ => ()
        }
    }

    // If the expression finishes with a literal, the buffer will contain it
    if !(literal_buffer.is_empty()) {
        pop_literal_buffer_to_tokens(&mut literal_buffer, &mut tokens);
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::token::literal::Literal;
    use super::token::operator::Operator;

    #[test]
    fn tokenize_creates_tokens_from_a_string() {
        let string = "2*3";
        let result: Vec<Token> = tokenize(string);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("2"))),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::from(String::from("3")))
        ].to_vec();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn tokenize_deals_with_multiple_digits_numbers() {
        let string = "22*33";
        let result: Vec<Token> = tokenize(string);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("22"))),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::from(String::from("33")))
        ].to_vec();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn tokenize_deals_with_parentheses() {
        let string = "2*(4-3)";
        let result: Vec<Token> = tokenize(string);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("2"))),
            Token::Operator(Operator::Times),
            Token::LeftParenthesis,
            Token::Literal(Literal::from(String::from("4"))),
            Token::Operator(Operator::Minus),
            Token::Literal(Literal::from(String::from("3"))),
            Token::RightParenthesis
        ].to_vec();

       assert_eq!(result, expected_result);
    }
}
