use std::path::PathBuf;

use once_cell::sync::Lazy;
pub(crate) static PORT: Lazy<u16> = Lazy::new(|| {
    option_env!("PORT")
        .unwrap_or("8080")
        .parse::<u16>()
        .expect("PORT must be a valid 16-bit unsigned integer")
});
pub(crate) static HOST: Lazy<&str> = Lazy::new(|| option_env!("HOST").unwrap_or("127.0.0.1"));
pub(crate) static ENVIRONMENT: Lazy<&str> =
    Lazy::new(|| option_env!("ENVIRONMENT").unwrap_or("development"));
pub(crate) static FRONTEND_DIST: Lazy<PathBuf> = Lazy::new(|| {
    option_env!("FRONTEND_DIST")
        .unwrap_or("./dist")
        .parse::<PathBuf>()
        .expect("Invalid path format")
});

pub(crate) static PRODUCTION_ENV: &str = "production";
