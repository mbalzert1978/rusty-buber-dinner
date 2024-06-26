mod auth;
mod error;
mod health;
mod prelude;

use application::authentication_dependencies;
use axum::Extension;
use infrastructure::infrastructure_dependencies;

use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let config = infrastructure::get_configuration()?;
    let health = health::routes();
    let auth = auth::routes();

    let routes = axum::Router::new().merge(health).merge(auth);
    let (jwt, ids) = infrastructure_dependencies(config);

    let app = axum::Router::new()
        .nest("/api/v1/", routes)
        .layer(Extension(authentication_dependencies(ids, jwt)));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Bind failed.");

    axum::serve(listener, app).await.expect("Serve failed.");
    Ok(())
}
