use std::fmt::Display;

#[derive(Debug)]
pub enum ApplicationError {
    Generic,
    InfrastructureError(Box<dyn std::error::Error>),
    DomainError(Box<dyn std::error::Error>),
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ApplicationError {}
