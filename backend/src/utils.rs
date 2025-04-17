use crate::constants::{ENVIRONMENT, PRODUCTION_ENV};

#[inline]
pub fn is_production() -> bool {
    *ENVIRONMENT == PRODUCTION_ENV
}
