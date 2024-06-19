use crate::abstractions::Authentication;
use crate::prelude::*;

pub struct AuthenticationResult {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub token: String,
}

impl AuthenticationResult {
    pub(crate) fn new(id: String, first_name: String, last_name: String, email: String, token: String) -> Self {
        Self {
            id,
            first_name,
            last_name,
            email,
            token,
        }
    }
}

pub(crate) struct AuthService<T> {
    id_generator: IdProvider<T>,
    token_generator: JwtTokenGenerator,
}

impl<T> AuthService<T> {
    pub(crate) fn new(id_generator: IdProvider<T>, token_generator: JwtTokenGenerator) -> Self {
        Self {
            id_generator,
            token_generator,
        }
    }
}

impl<T> Authentication for AuthService<T>
where
    T: 'static + ToString,
{
    fn login(&self, email: &str, password: &str) -> AuthenticationResult {
        let user_id = self.id_generator.generate_id().to_string();
        AuthenticationResult::new(
            user_id,
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
        let user_id = self.id_generator.generate_id().to_string();
        let token = self
            .token_generator
            .generate_token(&user_id, first_name, last_name)
            .unwrap();

        AuthenticationResult::new(
            user_id,
            first_name.to_string(),
            last_name.to_string(),
            email.to_string(),
            token,
        )
    }
}
