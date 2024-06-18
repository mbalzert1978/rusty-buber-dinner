use std::fmt::Display;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum InfrastructurError {
    Generic,
}

impl std::error::Error for InfrastructurError {}

impl Display for InfrastructurError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
