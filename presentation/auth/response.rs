use application::AuthenticationResult;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct AuthenticationResponse {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    token: String,
}

impl From<AuthenticationResult> for AuthenticationResponse {
    fn from(value: AuthenticationResult) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            token: value.token,
        }
    }
}
