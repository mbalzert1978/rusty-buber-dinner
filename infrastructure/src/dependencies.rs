use std::sync::Arc;

use application::abstractions::IdProvider;
use application::abstractions::JwtTokenGenerator;

use crate::authentication::jwt::JwtGenerator;
use crate::id::IdGenerator;

pub fn infrastructure_dependencies() -> (Arc<dyn IdProvider<Id = uuid::Uuid>>, Arc<dyn JwtTokenGenerator>) {
    let id_provider = Arc::new(IdGenerator) as Arc<dyn IdProvider<Id = uuid::Uuid>>;
    let token_generator = Arc::new(JwtGenerator) as Arc<dyn JwtTokenGenerator>;
    (id_provider, token_generator)
}
