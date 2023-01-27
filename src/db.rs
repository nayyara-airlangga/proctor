use std::{env::var, fmt::Display};

use sqlx::{error::BoxDynError, migrate::MigrateError, postgres::PgPoolOptions, PgPool};

#[derive(Debug)]
pub enum DbError {
    ZeroConn,
    NoUrl,
    SqlxError(String),
}

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroConn => write!(f, "Pool connections can't be zero"),
            Self::NoUrl => write!(f, "DATABASE_URL must be provided"),
            Self::SqlxError(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for DbError {}

pub async fn connect_to_db(num: u32) -> Result<PgPool, BoxDynError> {
    if num == 0 {
        tracing::error!("{}", DbError::ZeroConn);
        return Err(Box::new(DbError::ZeroConn));
    }

    let db_url = var("DATABASE_URL").map_err(|_| {
        tracing::error!("{}", DbError::NoUrl);
        Box::new(DbError::NoUrl)
    })?;

    Ok(PgPoolOptions::new()
        .max_connections(num)
        .connect(&db_url)
        .await
        .map_err(|err| {
            tracing::error!("{err}");
            Box::new(DbError::SqlxError(err.to_string()))
        })?)
}

pub async fn apply_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    if !cfg!(debug_assertions) {
        tracing::info!("Applying migrations...");

        sqlx::migrate!()
            .run(pool)
            .await
            .map(|_| tracing::info!("Migrations applied!"))
            .map_err(|err| {
                tracing::error!("{err}");
                err
            })
    } else {
        tracing::info!("Running in debug mode, migrations will not be applied automatically");
        Ok(())
    }
}
