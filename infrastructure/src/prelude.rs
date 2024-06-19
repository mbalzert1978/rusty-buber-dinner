use std::sync::Arc;

use application::abstractions::DateTimeProvider as _DateTimeProvider;
use application::ApplicationError;

pub(crate) type Result<T> = core::result::Result<T, ApplicationError>;
pub(crate) type DateTimeProvider<T> = Arc<dyn _DateTimeProvider<DateTime = T>>;
