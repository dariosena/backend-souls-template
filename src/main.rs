use std::net::SocketAddr;
use axum::{Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = SocketAddr::from(([127, 0, 0, 1], {{port}}));

    let api = Router::new().route("/api", get(health_endpoint));

    axum::Server::bind(&address).serve(api.into_make_service()).await?;

    Ok(())
}

async fn health_endpoint() -> impl IntoResponse {
    (StatusCode::OK, "API service health!")
}
