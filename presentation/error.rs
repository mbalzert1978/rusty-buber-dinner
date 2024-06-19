use std::fmt::Display;

use application::ApplicationError;

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

impl From<ApplicationError> for Error {
    fn from(value: ApplicationError) -> Self {
        Self::Generic(value.to_string())
    }
}
