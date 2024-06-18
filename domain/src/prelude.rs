use crate::error::DomainError;
pub(crate) type Result<T> = core::result::Result<T, DomainError>;
