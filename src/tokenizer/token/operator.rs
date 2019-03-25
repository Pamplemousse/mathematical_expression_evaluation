use std::ops::{Add, Div, Mul, Sub};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Error};

#[derive(Clone)]
pub enum Operator {
    Plus, Minus, Times, Slash, Caret
}

impl Operator {
    pub fn call(&self, first_operand: f32, second_operand: f32) -> f32 {
        let operator: fn(f32, f32) -> f32 = match *self {
            Operator::Plus => <f32 as Add>::add,
            Operator::Times => <f32 as Mul>::mul,
            Operator::Minus => <f32 as Sub>::sub,
            Operator::Slash => <f32 as Div>::div,
            Operator::Caret => f32::powf,
        };
        operator(first_operand, second_operand)
    }

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
        match *self {
            Operator::Plus => '+',
            Operator::Minus => '-',
            Operator::Times => '*',
            Operator::Slash => '/',
            Operator::Caret => '^'
        }
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

    #[test]
    fn operator_plus_computes_an_addition() {
        let first_operand: [f32; 2] = [1.0, 2.0];
        let second_operand: [f32; 2] = [1.0, 1.0];

        let result: Vec<f32> = first_operand
            .iter()
            .zip(second_operand.iter())
            .map(|(c1, c2)| Operator::Plus.call(*c1, *c2))
            .collect();
        let expected_result: [f32; 2] = [2.0, 3.0];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn operator_minus_computes_a_substraction() {
        let first_operand: [f32; 2] = [1.0, 2.0];
        let second_operand: [f32; 2] = [1.0, 1.0];

        let result: Vec<f32> = first_operand
            .iter()
            .zip(second_operand.iter())
            .map(|(c1, c2)| Operator::Minus.call(*c1, *c2))
            .collect();
        let expected_result: [f32; 2] = [0.0, 1.0];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn operator_times_computes_a_multiplication() {
        let first_operand: [f32; 3] = [1.0, 2.0, 3.0];
        let second_operand: [f32; 3] = [1.0, 1.0, 2.0];

        let result: Vec<f32> = first_operand
            .iter()
            .zip(second_operand.iter())
            .map(|(c1, c2)| Operator::Times.call(*c1, *c2))
            .collect();
        let expected_result: [f32; 3] = [1.0, 2.0, 6.0];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn operator_slash_computes_a_division() {
        let first_operand: [f32; 3] = [1.0, 1.0, 3.0];
        let second_operand: [f32; 3] = [1.0, 2.0, 2.0];

        let result: Vec<f32> = first_operand
            .iter()
            .zip(second_operand.iter())
            .map(|(c1, c2)| Operator::Slash.call(*c1, *c2))
            .collect();
        let expected_result: [f32; 3] = [1.0, 0.5, 1.5];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn operator_caret_computes_a_exponentiation() {
        let first_operand: [f32; 3] = [2.0, 2.0, 2.0];
        let second_operand: [f32; 3] = [1.0, 2.0, 3.0];

        let result: Vec<f32> = first_operand
            .iter()
            .zip(second_operand.iter())
            .map(|(c1, c2)| Operator::Caret.call(*c1, *c2))
            .collect();
        let expected_result: [f32; 3] = [2.0, 4.0, 8.0];

        assert_eq!(result, expected_result);
    }
}
