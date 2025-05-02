use crate::constants::{ENVIRONMENT, POSTGRESQL_URL, PRODUCTION_ENV};
use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    Pool, Postgres,
};

#[inline]
pub fn is_production() -> bool {
    *ENVIRONMENT == PRODUCTION_ENV
}

pub async fn init_db() -> Result<Pool<Postgres>, std::io::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(*POSTGRESQL_URL)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let pool_ref = &pool;

    let query = async |q: &str| -> Result<PgQueryResult, std::io::Error> {
        let result = sqlx::query(q)
            .execute(pool_ref)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(result)
    };

    query(r#"CREATE EXTENSION IF NOT EXISTS "pgcrypto";"#).await?;
    query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );
        "#,
    )
    .await?;
    query(
        r#"
    CREATE TABLE IF NOT EXISTS shortened_urls (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        original_url TEXT NOT NULL,
        short_url TEXT NOT NULL UNIQUE,

        expiry_date TIMESTAMPTZ,
        created_at TIMESTAMPTZ NOT NULL,
        updated_at TIMESTAMPTZ NOT NULL,

        owner UUID NOT NULL REFERENCES users(id),
        redirects BIGINT NOT NULL DEFAULT 0
    );
    "#,
    )
    .await?;
    Ok(pool)
}
