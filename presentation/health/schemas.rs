use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct HealthResponse {
    healthy: bool,
}

impl HealthResponse {
    pub(crate) fn new() -> Self {
        Self { healthy: true }
    }
}

impl IntoResponse for HealthResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
