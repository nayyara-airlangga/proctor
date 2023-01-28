use std::borrow::Cow;

use fancy_regex::Regex;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterUserRequest {
    #[validate(email)]
    #[schema(example = "example@email.com")]
    pub email: String,

    #[validate(custom = "validate_password")]
    #[schema(example = "Very string and secure password")]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let password_regex =
        Regex::new(r"^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,24}$").unwrap();

    if !password_regex.is_match(password).unwrap() {
        let mut err = ValidationError::new("invalid_password");
        err.message = Some(Cow::from("Password must contain at least 1 number, 1 lower and uppercase letter, 1 special character, and have 8-24 characters"));
        return Err(err);
    }

    Ok(())
}