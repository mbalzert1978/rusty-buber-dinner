use std::fmt::Display;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum InfrastructureError {
    Generic,
}

impl std::error::Error for InfrastructureError {}

impl Display for InfrastructureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
