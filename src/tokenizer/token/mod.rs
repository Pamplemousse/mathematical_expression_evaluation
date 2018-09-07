use std::fmt::{Debug, Display, Formatter, Error};

pub mod operator;
use self::operator::Operator;

pub mod digit;
use self::digit::Digit;

pub fn is_digit(c: char) -> bool {
    match Digit::new(c) {
        Some(_) => true,
        None => false
    }
}

pub fn is_operator(c: char) -> bool {
    match Operator::new(c) {
        Some(_) => true,
        None => false
    }
}

pub struct Token {
    pub kind: String,
    pub value: String
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_digit_returns_true_only_when_tested_character_is_a_digit() {
        let result: Vec<bool> = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            'a', '+', ','
        ]
            .iter()
            .map(|c| is_digit(*c))
            .collect();

        let expected_result: [bool; 13] = [
            true, true, true, true, true, true, true, true, true, true,
            false, false, false
        ];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn is_operator_returns_true_only_when_tested_character_is_an_operator() {
        let result: Vec<bool> = [
            '+', '-', '*', '/', '^',
            'a', '1', ','
        ]
            .iter()
            .map(|c| is_operator(*c))
            .collect();

        let expected_result: [bool; 8] = [
            true, true, true, true, true,
            false, false, false
        ];

        assert_eq!(result, expected_result);
    }
}
