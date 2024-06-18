use crate::error::ApplicationError;
pub(crate) type Result<T> = std::result::Result<T, ApplicationError>;
