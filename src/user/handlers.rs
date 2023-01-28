use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

use super::{errors::RegisterUserError, requests::RegisterUserRequest, UserService};

#[utoipa::path(
    post,
    path = "/users/register",
    request_body = RegisterUserRequest,
    responses(
        (status = 201, body = RegisterUserResponse),
        (status = 400, body = ApiErrors),
        (status = 422, body = ApiErrors),
        (status = 500, body = ApiErrors)
    )
)]
pub async fn register_user(
    State(user_service): State<UserService>,
    Json(body): Json<RegisterUserRequest>,
) -> impl IntoResponse {
    match user_service.register_user(body).await {
        Ok(user) => (StatusCode::CREATED, Json(json!(user))),
        Err(RegisterUserError::EmailIsTaken) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"errors": ["Email is already taken"]})),
        ),
        Err(RegisterUserError::InternalServerError) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"errors": ["Something went wrong"]})),
        ),
        Err(RegisterUserError::BadRequestError(errs)) => {
            (StatusCode::BAD_REQUEST, Json(json!({ "errors": errs })))
        }
    }
}
