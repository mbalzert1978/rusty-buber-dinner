use std::sync::Arc;

use crate::error::ApplicationError;
use crate::services::authentication::abstractions::Authentication as _Authentication;
use crate::services::authentication::abstractions::JwtTokenGenerator as _JwtTokenGenerator;

pub(crate) type Result<T> = std::result::Result<T, ApplicationError>;
pub(crate) type JwtTokenGenerator = Arc<dyn _JwtTokenGenerator>;
pub type AuthenticationService = Arc<dyn _Authentication>;
