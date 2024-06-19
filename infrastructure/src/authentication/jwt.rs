use crate::config::jwt_configuration::JwtConfig;
use crate::prelude::*;

use application::abstractions::JwtTokenGenerator;
use application::ApplicationError;
use application::IdProvider;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub(crate) sub: String,
    pub(crate) given_name: String,
    pub(crate) family_name: String,
    pub(crate) jtid: String,
    pub(crate) exp: usize,
    pub(crate) iss: String,
    pub(crate) aud: String,
}

pub(crate) struct JwtGenerator<T: JwtConfig, I> {
    id_generator: IdProvider<I>,
    jwt_config: T,
    datetime_provider: DateTimeProvider<chrono::DateTime<chrono::offset::Utc>>,
}

impl<T, I> JwtGenerator<T, I>
where
    T: JwtConfig,
    I: 'static + ToString,
{
    pub(crate) fn new(
        id_generator: IdProvider<I>,
        config: T,
        datetime_provider: DateTimeProvider<chrono::DateTime<chrono::offset::Utc>>,
    ) -> Self {
        Self {
            id_generator,
            jwt_config: config,
            datetime_provider,
        }
    }
}

impl<T, I> JwtTokenGenerator for JwtGenerator<T, I>
where
    T: JwtConfig,
    I: 'static + ToString,
{
    fn generate_token(&self, id: &str, first_name: &str, last_name: &str) -> Result<String> {
        let claims = Claims {
            sub: id.to_string(),
            given_name: first_name.to_string(),
            family_name: last_name.to_string(),
            jtid: self.id_generator.generate_id().to_string(),
            exp: self
                .datetime_provider
                .add_minutes(self.jwt_config.expiration_minutes())
                .timestamp() as usize,
            iss: self.jwt_config.issuer().to_string(),
            aud: self.jwt_config.audience().to_string(),
        };
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.jwt_config.secret_key().as_ref()),
        )
        .map_err(|e| ApplicationError::InfrastructureError(Box::new(e)))
    }
}
