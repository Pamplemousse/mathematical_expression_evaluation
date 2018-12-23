use std::fmt::{Display, Formatter, Error};

pub mod digit;
use self::digit::Digit;

#[derive(Clone, Debug)]
pub struct Literal(Vec<Digit>);

impl Literal {
    pub fn new(digits: Vec<Digit>) -> Literal {
        return Literal(digits);
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let string: String = self.0
            .iter()
            .map(|digit| digit.to_char())
            .collect();
        write!(f, "{}", string)
    }
}

impl From<String> for Literal {
    fn from(string: String) -> Self {
        Literal::new(
            string
                .chars()
                .map(|c| Digit::new(c))
                .filter(|c| c.is_some())
                .map(|c| c.unwrap())
                .collect()
        )
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_literal_can_be_represented_as_a_string() {
        let result: Vec<String> = [
            [Digit::One].to_vec(),
            [Digit::Two, Digit::Three].to_vec(),
            [Digit::Four, Digit::Five, Digit::Six].to_vec(),
            [Digit::Seven, Digit::Eight, Digit::Nine, Digit::Zero].to_vec()
        ]
            .iter()
            .cloned()
            .map(|vector| Literal::new(vector))
            .map(|literal| literal.to_string())
            .collect();

        let expected_result: Vec<String> = ["1", "23", "456", "7890"]
            .iter()
            .map(|&string| String::from(string))
            .collect();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn literals_can_be_instanciated_from_strings() {
        let result: Vec<Literal> = ["1", "23", "456", "7890"]
            .iter()
            .map(|&string| String::from(string))
            .map(|string| Literal::from(string))
            .collect();

        let expected_result: Vec<Literal> = [
            [Digit::One].to_vec(),
            [Digit::Two, Digit::Three].to_vec(),
            [Digit::Four, Digit::Five, Digit::Six].to_vec(),
            [Digit::Seven, Digit::Eight, Digit::Nine, Digit::Zero].to_vec()
        ]
            .iter()
            .cloned()
            .map(|vector| Literal::new(vector))
            .collect();

        assert_eq!(result, expected_result);
    }
}
