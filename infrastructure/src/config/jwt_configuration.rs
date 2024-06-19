use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct JwtSettings {
    #[envconfig(from = "SECRET_KEY")]
    secret_key: String,
    #[envconfig(from = "ISSUER")]
    issuer: String,
    #[envconfig(from = "AUDIENCE")]
    audience: String,
    #[envconfig(from = "EXPIRATION_MINUTES", default = "60")]
    expiration_minutes: i64,
}

impl JwtConfig for JwtSettings {
    fn secret_key(&self) -> &str {
        &self.secret_key
    }

    fn issuer(&self) -> &str {
        &self.issuer
    }

    fn audience(&self) -> &str {
        &self.audience
    }

    fn expiration_minutes(&self) -> i64 {
        self.expiration_minutes
    }
}

pub trait JwtConfig: 'static + Sync + Send {
    fn secret_key(&self) -> &str;
    fn issuer(&self) -> &str;
    fn audience(&self) -> &str;
    fn expiration_minutes(&self) -> i64;
}
