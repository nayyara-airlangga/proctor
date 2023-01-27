use axum::{routing::get, Router, Server};
use dotenv::dotenv;
use std::{env::var, net::SocketAddr, str::FromStr};
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::{info, Level};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use modules::{docs::ApiDoc, index::health_check, trace::init_tracer};

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    dotenv().ok();

    init_tracer();

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(health_check))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        );

    let host = var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = SocketAddr::from_str(&format!("{host}:{port}"))?;

    info!("Server listening at {addr}");
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
