pub(crate) mod api_configuration;
pub(crate) mod jwt_configuration;
pub use jwt_configuration::JwtConfig;
pub(crate) use jwt_configuration::JwtSettings;

use crate::prelude::*;
use envconfig::Envconfig;

pub fn get_configuration() -> Result<JwtSettings> {
    dotenvy::dotenv().ok();
    JwtSettings::init_from_env().map_err(|e| application::ApplicationError::InfrastructureError(Box::new(e)))
}
