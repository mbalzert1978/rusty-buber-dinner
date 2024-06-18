use std::fmt::Display;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DomainError {
    Generic,
}

impl std::error::Error for DomainError {}

impl Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
