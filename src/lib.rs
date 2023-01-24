pub mod docs;

pub mod index {
    use axum::response::IntoResponse;

    #[utoipa::path(get, path = "/", responses((status = 200, description = "Server is running", body = String)))]
    pub async fn health_check() -> impl IntoResponse {
        "OK!"
    }
}
