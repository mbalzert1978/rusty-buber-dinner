use std::sync::Arc;

use crate::abstractions::Authentication as _Authentication;
use crate::abstractions::IdProvider as _IdProvider;
use crate::abstractions::JwtTokenGenerator as _JwtTokenGenerator;

pub(crate) type JwtTokenGenerator = Arc<dyn _JwtTokenGenerator>;
pub type IdProvider<T> = Arc<dyn _IdProvider<Id = T>>;
pub type AuthenticationService = Arc<dyn _Authentication>;
