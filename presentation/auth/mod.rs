use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

pub(crate) fn routes() -> Router {
    let auth_routes = Router::new()
        .route("/register", post(register_api))
        .route("/login", post(login_api));

    Router::new().nest("/auth", auth_routes)
}

async fn register_api() -> impl IntoResponse {
    StatusCode::OK
}

async fn login_api() -> impl IntoResponse {
    StatusCode::OK
}
