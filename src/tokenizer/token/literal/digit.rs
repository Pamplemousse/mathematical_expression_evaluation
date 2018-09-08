use std::fmt::{Debug, Display, Formatter, Error};

#[derive(Clone)]
pub enum Digit {
    Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine
}

impl Digit {
    pub fn new(c: char) -> Option<Digit> {
        match c {
            '0' => Some(Digit::Zero),
            '1' => Some(Digit::One),
            '2' => Some(Digit::Two),
            '3' => Some(Digit::Three),
            '4' => Some(Digit::Four),
            '5' => Some(Digit::Five),
            '6' => Some(Digit::Six),
            '7' => Some(Digit::Seven),
            '8' => Some(Digit::Eight),
            '9' => Some(Digit::Nine),
            _ => None
        }
    }

    pub fn to_char(&self) -> char {
        let operator: char = match *self {
            Digit::Zero => '0',
            Digit::One => '1',
            Digit::Two => '2',
            Digit::Three => '3',
            Digit::Four => '4',
            Digit::Five => '5',
            Digit::Six => '6',
            Digit::Seven => '7',
            Digit::Eight => '8',
            Digit::Nine => '9',
        };
        return operator;
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.to_char())
    }
}

impl Debug for Digit {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let enum_name: &str = match *self {
            Digit::Zero => "Digit::Zero",
            Digit::One => "Digit::One",
            Digit::Two => "Digit::Two",
            Digit::Three => "Digit::Three",
            Digit::Four => "Digit::Four",
            Digit::Five => "Digit::Five",
            Digit::Six => "Digit::Six",
            Digit::Seven => "Digit::Seven",
            Digit::Eight => "Digit::Eight",
            Digit::Nine => "Digit::Nine"
        };
        write!(f, "{} ({})", enum_name, self.to_char())
    }
}

impl PartialEq for Digit {
    fn eq(&self, other: &Digit) -> bool {
        self.to_char() == other.to_char()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_digit_has_a_character_representation() {
        let result: String = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            Digit::Zero, Digit::One, Digit::Two, Digit::Three, Digit::Four,
            Digit::Five, Digit::Six, Digit::Seven, Digit::Eight, Digit::Nine
        );
        let expected_result: &str = "0123456789";

        assert_eq!(result, expected_result);
    }

    #[test]
    fn operators_can_be_instanciated_from_characters() {
        let result: Vec<Digit> = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
            .iter()
            .map(|c| Digit::new(*c))
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect();

        let expected_result: [Digit; 10] = [
            Digit::Zero, Digit::One, Digit::Two, Digit::Three, Digit::Four,
            Digit::Five, Digit::Six, Digit::Seven, Digit::Eight, Digit::Nine,
        ];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn operator_constructor_return_none_if_character_is_not_an_operator() {
        let result: Vec<Digit> = ['a', '+', ',']
            .iter()
            .map(|c| Digit::new(*c))
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect();

        assert_eq!(result.is_empty(), true);
    }
}
