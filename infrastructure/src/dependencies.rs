use application::JwtTokenGenerator;
use std::sync::Arc;

use crate::authentication::jwt::JwtGenerator;

pub type JwtService = Arc<dyn JwtTokenGenerator>;

pub fn infrastructure_dependencies() -> JwtService {
    Arc::new(JwtGenerator) as JwtService
}
