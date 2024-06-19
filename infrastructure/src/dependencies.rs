use std::sync::Arc;

use application::abstractions::DateTimeProvider;
use application::abstractions::IdProvider;
use application::abstractions::JwtTokenGenerator;
use chrono::DateTime as _DateTime;

use crate::authentication::jwt::JwtGenerator;
use crate::config::jwt_configuration::JwtSettings;
use crate::provider::DateTime;
use crate::provider::IdGenerator;

type InfrastructureDependencies = (Arc<dyn JwtTokenGenerator>, Arc<dyn IdProvider<Id = uuid::Uuid>>);

pub fn infrastructure_dependencies(jwt_settings: JwtSettings) -> InfrastructureDependencies {
    let id_provider = Arc::new(IdGenerator) as Arc<dyn IdProvider<Id = uuid::Uuid>>;
    let datetime_provider = Arc::new(DateTime) as Arc<dyn DateTimeProvider<DateTime = _DateTime<chrono::offset::Utc>>>;
    let token_generator =
        Arc::new(JwtGenerator::new(id_provider.clone(), jwt_settings, datetime_provider)) as Arc<dyn JwtTokenGenerator>;
    (token_generator, id_provider)
}
