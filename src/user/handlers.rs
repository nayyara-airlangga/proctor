use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

use super::{errors::RegisterUserError, requests::RegisterUserRequest, UserService};

pub async fn register_user(
    State(user_service): State<UserService>,
    Json(body): Json<RegisterUserRequest>,
) -> impl IntoResponse {
    match user_service.register_user(body).await {
        Ok(user) => (StatusCode::CREATED, Json(json!(user))),
        Err(RegisterUserError::EmailIsTaken) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Email is already taken"})),
        ),
        Err(RegisterUserError::InternalServerError) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Something went wrong"})),
        ),
        Err(RegisterUserError::BadRequestError(errs)) => {
            (StatusCode::BAD_REQUEST, Json(json!({ "errors": errs })))
        }
    }
}
