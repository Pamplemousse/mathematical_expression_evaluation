use std::fmt::{Debug, Display, Formatter, Error};

pub mod operator;
use self::operator::Operator;

pub mod literal;
use self::literal::Literal;

fn is_left_parenthesis(c: char) -> bool {
    c == '('
}

fn is_right_parenthesis(c: char) -> bool {
    c == ')'
}

#[derive(Clone, PartialEq)]
pub enum Token {
    Literal(Literal),
    Operator(Operator),
    LeftParenthesis,
    RightParenthesis
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Token::LeftParenthesis => write!(f, "("),
            Token::RightParenthesis => write!(f, ")"),
            Token::Literal(ref literal) => <Literal as Display>::fmt(&literal, f),
            Token::Operator(ref operator) => <Operator as Display>::fmt(&operator, f)
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Token::LeftParenthesis => write!(f, "LeftParenthesis ("),
            Token::RightParenthesis => write!(f, "RightParenthesis )"),
            Token::Literal(ref literal) => <Literal as Debug>::fmt(&literal, f),
            Token::Operator(ref operator) => <Operator as Debug>::fmt(&operator, f)
        }
    }
}

impl Token {
    pub fn from_char(character: char) -> Option<Token> {
        match character {
            character if is_left_parenthesis(character) => {
                Some(Token::LeftParenthesis)
            },
            character if is_right_parenthesis(character) => {
                Some(Token::RightParenthesis)
            }
            _ => {
                let operator: Option<Operator> = Operator::new(character);
                match operator {
                    Some(operator) => Some(Token::Operator(operator)),
                    None => None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_token_can_be_represented_as_a_string() {
        let _1: String = String::from("1");
        let _23: String = String::from("23");
        let _456: String = String::from("456");
        let _7890: String = String::from("7890");

        let result: Vec<String> = [
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Minus),
            Token::Operator(Operator::Times),
            Token::Operator(Operator::Slash),
            Token::Operator(Operator::Caret),
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::Literal(Literal::from(_1)),
            Token::Literal(Literal::from(_23)),
            Token::Literal(Literal::from(_456)),
            Token::Literal(Literal::from(_7890))
        ]
        .iter()
        .map(|token| token.to_string())
        .collect();

        let expected_result: Vec<String> =
            ["+", "-", "*", "/", "^", "(", ")", "1", "23", "456", "7890"]
                .iter()
                .map(|string| String::from(*string))
                .collect();

        assert_eq!(result, expected_result);
    }

    #[test]
    // LeftParenthesis, RightParenthesis, Operator are usually represented by
    // single characters.
    fn some_tokens_can_be_instanciated_from_a_character() {
        let result: Vec<Token> = [
            '+', '-', '*', '/', '^', '(', ')'
        ]
        .iter()
        .map(|character| Token::from_char(*character))
        .filter(|character| character.is_some())
        .map(|character| character.unwrap())
        .collect();

        let expected_result: [Token; 7] = [
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Minus),
            Token::Operator(Operator::Times),
            Token::Operator(Operator::Slash),
            Token::Operator(Operator::Caret),
            Token::LeftParenthesis,
            Token::RightParenthesis,
        ];

        assert_eq!(result, expected_result);
    }
}
