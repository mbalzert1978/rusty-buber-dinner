use crate::prelude::*;
use crate::services::authentication::AuthService;

use std::sync::Arc;

pub fn authentication_dependencies(token_generator: JwtTokenGenerator) -> AuthenticationService {
    Arc::new(AuthService::new(token_generator)) as AuthenticationService
}
