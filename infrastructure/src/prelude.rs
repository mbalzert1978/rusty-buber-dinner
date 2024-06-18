use crate::error::InfrastructurError;
pub(crate) type Result<T> = core::result::Result<T, InfrastructurError>;
