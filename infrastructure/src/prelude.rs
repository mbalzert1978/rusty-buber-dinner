use application::ApplicationError;
pub(crate) type Result<T> = core::result::Result<T, ApplicationError>;
