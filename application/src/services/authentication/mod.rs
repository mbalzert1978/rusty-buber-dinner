use crate::prelude::*;
use abstractions::Authentication;
pub(crate) mod abstractions;

pub struct AuthenticationResult {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub token: String,
}

impl AuthenticationResult {
    pub(crate) fn new(first_name: String, last_name: String, email: String, token: String) -> Self {
        Self {
            id: uuid::Uuid::now_v7().to_string(),
            first_name,
            last_name,
            email,
            token,
        }
    }
}

pub(crate) struct AuthService {
    token_generator: JwtTokenGenerator,
}

impl AuthService {
    pub(crate) fn new(token_generator: JwtTokenGenerator) -> Self {
        Self { token_generator }
    }
}

impl Authentication for AuthService {
    fn login(&self, email: &str, password: &str) -> AuthenticationResult {
        AuthenticationResult::new(
            "first_name".to_string(),
            "last_name".to_string(),
            email.to_string(),
            "token".to_string(),
        )
    }

    fn register(&self, first_name: &str, last_name: &str, email: &str, password: &str) -> AuthenticationResult {
        // check if user exists

        // create user

        // generate token
        let user_id = uuid::Uuid::now_v7().to_string();
        let token = self
            .token_generator
            .generate_token(&user_id, first_name, last_name)
            .unwrap();

        AuthenticationResult::new(first_name.to_string(), last_name.to_string(), email.to_string(), token)
    }
}
