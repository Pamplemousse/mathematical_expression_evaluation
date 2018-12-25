use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Error};

#[derive(Clone)]
pub enum Operator {
    Plus, Minus, Times, Slash, Caret
}

impl Operator {
    pub fn new(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Plus),
            '-' => Some(Operator::Minus),
            '*' => Some(Operator::Times),
            '/' => Some(Operator::Slash),
            '^' => Some(Operator::Caret),
            _ => None
        }
    }

    pub fn to_char(&self) -> char {
        let operator: char = match *self {
            Operator::Plus => '+',
            Operator::Minus => '-',
            Operator::Times => '*',
            Operator::Slash => '/',
            Operator::Caret => '^'
        };
        return operator;
    }

    fn priority(&self) -> u32 {
        match *self {
            Operator::Plus | Operator::Minus => 1,
            Operator::Times | Operator::Slash => 2,
            Operator::Caret => 3
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.to_char())
    }
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let enum_name: &str = match *self {
            Operator::Plus => "Operator::Plus",
            Operator::Minus => "Operator::Minus",
            Operator::Times => "Operator::Times",
            Operator::Slash => "Operator::Slash",
            Operator::Caret => "Operator::Caret"
        };
        write!(f, "{} ({})", enum_name, self.to_char())
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Operator) -> bool {
        self.to_char() == other.to_char()
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Operator) -> Option<Ordering> {
        Some(
            self.priority().cmp(&other.priority())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_operator_has_a_character_representation() {
        let result: String = format!(
            "{}{}{}{}{}",
            Operator::Plus,
            Operator::Minus,
            Operator::Times,
            Operator::Slash,
            Operator::Caret
        );
        let expected_result: &str = "+-*/^";

        assert_eq!(result, expected_result);
    }

    #[test]
    fn operators_can_be_instanciated_from_characters() {
        let result: Vec<Operator> = ['+', '-', '*', '/', '^']
            .iter()
            .map(|c| Operator::new(*c))
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect();

        let expected_result: [Operator; 5] = [
            Operator::Plus,
            Operator::Minus,
            Operator::Times,
            Operator::Slash,
            Operator::Caret
        ];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn operator_constructor_return_none_if_character_is_not_an_operator() {
        let result: Vec<Operator> = ['a', '1', ',']
            .iter()
            .map(|c| Operator::new(*c))
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect();

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn operators_can_be_ordered_following_precedence_in_arithmetic() {
        let result: Vec<bool> = [
            (Operator::Caret > Operator::Times),
            (Operator::Caret > Operator::Slash),
            (Operator::Times > Operator::Plus),
            (Operator::Times > Operator::Minus),
            (Operator::Slash > Operator::Plus),
            (Operator::Slash > Operator::Minus),

            (Operator::Times < Operator::Caret),
            (Operator::Slash < Operator::Caret),
            (Operator::Plus < Operator::Times),
            (Operator::Minus < Operator::Times),
            (Operator::Plus < Operator::Slash),
            (Operator::Minus < Operator::Slash),

            (Operator::Times == Operator::Slash),
            (Operator::Plus == Operator::Minus),
            (Operator::Times > Operator::Slash),
            (Operator::Slash < Operator::Times),
            (Operator::Plus > Operator::Minus),
            (Operator::Minus < Operator::Plus)
        ].to_vec();

        let expected_result: [bool; 18] = [
            true, true, true, true, true, true,
            true, true, true, true, true, true,
            false, false, false, false, false, false
        ];

        assert_eq!(result, expected_result);
    }
}
