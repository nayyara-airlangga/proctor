use axum::{routing::get, Router, Server};
use dotenv::dotenv;
use std::{env::var, net::SocketAddr, str::FromStr};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use modules::{docs::ApiDoc, index::health_check};

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    dotenv().ok();

    if cfg!(debug_assertions) {
        tracing_subscriber::fmt().pretty().init();
    } else {
        tracing_subscriber::fmt().json().init();
    }

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(health_check))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let host = var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = SocketAddr::from_str(&format!("{host}:{port}"))?;

    info!("Server listening at {addr}");
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
