use crate::prelude::*;

use application::{ApplicationError, JwtTokenGenerator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub(crate) sub: String,
    pub(crate) given_name: String,
    pub(crate) family_name: String,
    pub(crate) jtid: String,
    pub(crate) exp: usize,
    pub(crate) iss: String,
}
#[derive(Default)]
pub(crate) struct JwtGenerator;

impl JwtTokenGenerator for JwtGenerator {
    fn generate_token(&self, id: &str, first_name: &str, last_name: &str) -> Result<String> {
        let signing_credentials = "super-secret-key".to_string();
        let claims = Claims {
            sub: id.to_string(),
            given_name: first_name.to_string(),
            family_name: last_name.to_string(),
            jtid: uuid::Uuid::now_v7().to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
            iss: "Rusty-Buber-dinner".to_string(),
        };
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(signing_credentials.as_ref()),
        )
        .map_err(|e| ApplicationError::InfrastructureError(Box::new(e)))
    }
}
