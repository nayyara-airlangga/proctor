use axum::{routing::post, Router};

use super::{handlers::register_user, UserService};

pub fn user_routes(user_service: UserService) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .with_state(user_service)
}
