use crate::constants::{ENVIRONMENT, POSTGRESQL_URL, PRODUCTION_ENV};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[inline]
pub fn is_production() -> bool {
    *ENVIRONMENT == PRODUCTION_ENV
}
