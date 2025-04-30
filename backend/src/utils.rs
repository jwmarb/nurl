use crate::constants::{ENVIRONMENT, PRODUCTION_ENV, POSTGRESQL_URL};
use sqlx::{PgPool, postgres::PgPoolOptions};

#[inline]
pub fn is_production() -> bool {
    *ENVIRONMENT == PRODUCTION_ENV
}


// connects to database
pub async fn connect_db() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(*POSTGRESQL_URL)
        .await?;

    Ok(pool)
}

// disconnects from database
pub async fn disconnect_db(pool: PgPool) {
    // Gracefully close the connection pool and wait for all connections to finish
    pool.close().await;
    println!("Database disconnected.");
}
