use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema)]
pub struct RegisterUserResponse {
    pub id: Uuid,
    #[schema(example = "example@email.com")]
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
