use once_cell::sync::Lazy;
use rand::Rng;

use std::path::PathBuf;
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

pub(crate) static NURL_SECRET: Lazy<String> = Lazy::new(|| {
    option_env!("NURL_SECRET")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            if *ENVIRONMENT == "development" {
                return "development-secret-key".to_string(); // Use 'a' for development to avoid issues with JWT encoding
            }
            let mut rng = rand::rng();
            let secret: String = (0..32) // Generate a 32-character secret
                .map(|_| {
                    let choices = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
                    let index = rng.random_range(0..choices.len());
                    choices.chars().nth(index).unwrap()
                })
                .collect();
            secret
        })
});

pub(crate) static PRODUCTION_ENV: &str = "production";

pub(crate) static POSTGRESQL_URL: Lazy<&str> = Lazy::new(|| {
    option_env!("POSTGRESQL_URL")
        .unwrap_or("postgresql://postgres:postgres@localhost:5432/postgres")
});
