use fancy_regex::Regex;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterUserRequest {
    #[validate(email(message = "Invalid email"))]
    #[schema(example = "example@email.com")]
    pub email: String,

    #[validate(custom(function = "validate_password", message = "Invalid password"))]
    #[schema(example = "Very string and secure password")]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let password_regex =
        Regex::new(r"^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,24}$").unwrap();

    if !password_regex.is_match(password).unwrap() {
        return Err(ValidationError::new("invalid_password"));
    }

    Ok(())
}
