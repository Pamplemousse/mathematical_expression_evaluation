use std::fmt::{Debug, Display, Formatter, Error};

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

