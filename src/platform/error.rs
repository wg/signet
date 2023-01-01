use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    System(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Self::System(error) = self;
        write!(f, "{}", error)
    }
}
