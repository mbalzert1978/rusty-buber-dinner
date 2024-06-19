mod auth;
mod error;
mod health;
mod prelude;

pub use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let health = health::routes();
    let auth = auth::routes();

    let routes = axum::Router::new().merge(health).merge(auth);

    let app = axum::Router::new().nest("/api/v1/", routes);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Bind failed.");

    axum::serve(listener, app).await.expect("Serve failed.");
    Ok(())
}