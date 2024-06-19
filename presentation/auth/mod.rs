use application::AuthenticationService;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Extension;
use axum::Json;
use axum::Router;
use request::LoginRequest;
use request::RegisterRequest;

mod request;
mod response;

pub(crate) fn routes() -> Router {
    let auth_routes = Router::new()
        .route("/register", post(register_api))
        .route("/login", post(login_api));

    Router::new().nest("/auth", auth_routes)
}

async fn register_api(
    Extension(authentication_service): Extension<AuthenticationService>,
    Json(request): Json<RegisterRequest>,
) -> impl IntoResponse {
    let auth_result = authentication_service.register(
        &request.first_name,
        &request.last_name,
        &request.email,
        &request.password,
    );
    let response = response::AuthenticationResponse::from(auth_result);
    Json(response).into_response()
}

async fn login_api(
    Extension(authentication_service): Extension<AuthenticationService>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    let auth_result = authentication_service.login(&request.email, &request.password);
    let response = response::AuthenticationResponse::from(auth_result);
    Json(response).into_response()
}
