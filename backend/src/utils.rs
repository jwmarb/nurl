use crate::constants::{DATABASE_URL, ENVIRONMENT, PRODUCTION_ENV};
use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    Pool, Postgres,
};

/// Checks if the application is running in production environment
/// 
/// # Returns
/// Boolean indicating if the environment is production
#[inline]
pub fn is_production() -> bool {
    *ENVIRONMENT == PRODUCTION_ENV
}

/// Initializes the database connection and sets up required tables
/// 
/// This function:
/// 1. Establishes a connection to the PostgreSQL database
/// 2. Creates the pgcrypto extension if it doesn't exist
/// 3. Creates the users table if it doesn't exist
/// 4. Creates the shortened_urls table if it doesn't exist
/// 
/// # Returns
/// Result containing the database connection pool
pub async fn init_db() -> Result<Pool<Postgres>, std::io::Error> {
    let pool = loop {
        match PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(1))
            .connect(DATABASE_URL.as_str())
            .await
        {
            Ok(pool) => break pool,
            Err(_) => {
                println!("Could not connect to database. Retrying...");
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            }
        }
    };

    println!("Successfully formed a DB connection");

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
            username TEXT NOT NULL UNIQUE,
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

/// Initializes a test database with a test user
/// 
/// This function is only available in test builds
/// 
/// # Returns
/// Database connection pool with test data
#[cfg(test)]
pub async fn init_test_db() -> Pool<Postgres> {
    let pool = init_db().await.unwrap();

    let pool_ref = &pool;

    sqlx::query(
        "INSERT INTO users (username, password) VALUES ($1, $2) ON CONFLICT (username) DO NOTHING",
    )
    .bind("test_user")
    .bind("test_password")
    .execute(pool_ref)
    .await
    .unwrap();

    pool
}
#[cfg(test)]
use crate::structs::User;

/// Retrieves the test user from the database
/// 
/// This function is only available in test builds
/// 
/// # Arguments
/// * `pool` - Database connection pool
/// 
/// # Returns
/// The test user
#[cfg(test)]
pub async fn get_test_user(pool: &Pool<Postgres>) -> User {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind("test_user")
        .fetch_one(pool)
        .await
        .unwrap()
}
