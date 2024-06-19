pub trait DateTimeProvider: 'static + Send + Sync {
    type DateTime;
    fn utc_now(&self) -> Self::DateTime;
    fn add_minutes(&self, duration: i64) -> Self::DateTime;
}

pub trait IdProvider: 'static + Send + Sync {
    type Id;
    fn generate_id(&self) -> Self::Id;
}
pub trait Authentication: 'static + Send + Sync {
    fn login(&self, email: &str, password: &str) -> super::AuthenticationResult;
    fn register(&self, first_name: &str, last_name: &str, email: &str, password: &str) -> super::AuthenticationResult;
}

pub trait JwtTokenGenerator: 'static + Send + Sync {
    fn generate_token(
        &self,
        id: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<String, crate::error::ApplicationError>;
}
