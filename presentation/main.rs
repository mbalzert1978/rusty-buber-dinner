mod auth;
mod error;
mod health;
mod prelude;

use application::authentication_dependencies;
use axum::Extension;
use infrastructure::infrastructure_dependencies;

pub use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let health = health::routes();
    let auth = auth::routes();

    let routes = axum::Router::new().merge(health).merge(auth);
    let (id_provider, token_generator) = infrastructure_dependencies();

    let app = axum::Router::new()
        .nest("/api/v1/", routes)
        .layer(Extension(authentication_dependencies(id_provider, token_generator)));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Bind failed.");

    axum::serve(listener, app).await.expect("Serve failed.");
    Ok(())
}
