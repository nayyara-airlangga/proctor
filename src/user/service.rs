use argon2;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use validator::Validate;

use super::{
    errors::RegisterUserError, models::User, requests::RegisterUserRequest, UserRepository,
};

#[derive(Clone)]
pub struct UserService {
    pub repository: UserRepository,
    hash_config: argon2::Config<'static>,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self {
            repository,
            hash_config: argon2::Config {
                variant: argon2::Variant::Argon2id,
                ..Default::default()
            },
        }
    }

    fn generate_password_hash(
        &self,
        password: impl AsRef<str>,
    ) -> Result<String, RegisterUserError> {
        let seed: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut salt = [0u8; 16];
        rng.fill_bytes(&mut salt);

        argon2::hash_encoded(password.as_ref().as_bytes(), &salt, &self.hash_config).map_err(
            |err| {
                tracing::error!("{err}");
                RegisterUserError::InternalServerError
            },
        )
    }

    fn _verify_password_hash(&self, password: impl AsRef<str>, hash: impl AsRef<str>) -> bool {
        argon2::verify_encoded(hash.as_ref(), password.as_ref().as_bytes()).unwrap()
    }

    pub async fn register_user(
        &self,
        body: RegisterUserRequest,
    ) -> Result<User, RegisterUserError> {
        body.validate().map_err(|err| {
            RegisterUserError::BadRequestError({
                let mut errors: Vec<String> = Vec::new();
                err.field_errors()
                    .into_values()
                    .for_each(|err| err.into_iter().for_each(|err| errors.push(err.to_string())));
                errors
            })
        })?;

        let password_hash = self.generate_password_hash(body.password)?;

        self.repository
            .create_user(body.email, password_hash)
            .await
            .map_err(|err| match err {
                sqlx::Error::Database(err) => {
                    if let Some(code) = err.code() {
                        if code == "23505" {
                            RegisterUserError::EmailIsTaken
                        } else {
                            tracing::error!("{err}");
                            RegisterUserError::InternalServerError
                        }
                    } else {
                        tracing::error!("{err}");
                        RegisterUserError::InternalServerError
                    }
                }
                _ => {
                    tracing::error!("{err}");
                    RegisterUserError::InternalServerError
                }
            })
    }
}
