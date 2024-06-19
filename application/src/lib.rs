mod dependencies;
mod error;
mod prelude;
mod services;

pub use crate::error::ApplicationError;
pub use crate::prelude::AuthenticationService;
pub use crate::services::authentication::abstractions::Authentication;
pub use crate::services::authentication::abstractions::JwtTokenGenerator;
pub use crate::services::authentication::AuthenticationResult;
pub use dependencies::authentication_dependencies;
