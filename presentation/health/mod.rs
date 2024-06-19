use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub(crate) mod schemas;

pub(crate) fn routes() -> Router {
    let health_route = Router::new().route("/", get(health_api));

    Router::new().nest("/health", health_route)
}

async fn health_api() -> impl IntoResponse {
    schemas::HealthResponse::new()
}
