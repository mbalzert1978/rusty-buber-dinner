use crate::prelude::*;
use crate::services::authentication::AuthService;

use std::sync::Arc;

pub fn authentication_dependencies<T>(
    id_provider: IdProvider<T>,
    token_generator: JwtTokenGenerator,
) -> AuthenticationService
where
    T: 'static + ToString,
{
    Arc::new(AuthService::new(id_provider, token_generator)) as AuthenticationService
}
