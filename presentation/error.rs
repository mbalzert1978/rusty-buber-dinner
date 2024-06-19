use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum Error {
    Generic(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for Error {}
