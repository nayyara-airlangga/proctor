use sqlx::{PgPool, Result};
use uuid::Uuid;

use super::models::User;

#[derive(Clone)]
pub struct UserRepository {
    pub pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(
        &self,
        email: impl AsRef<str>,
        password_hash: impl AsRef<str>,
    ) -> Result<User> {
        let query = r#"
        INSERT INTO users VALUES($1, $2, $3)
        RETURNING *
        "#;
        let id = Uuid::new_v4();

        sqlx::query_as(query)
            .bind(id)
            .bind(email.as_ref())
            .bind(password_hash.as_ref())
            .fetch_one(&self.pool)
            .await
    }
}
