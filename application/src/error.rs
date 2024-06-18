use std::fmt::Display;

#[derive(Debug)]
pub enum ApplicationError {
    Generic,
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ApplicationError {}
