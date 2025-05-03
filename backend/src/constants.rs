use once_cell::sync::Lazy;
use rand::Rng;

use std::path::PathBuf;
pub(crate) static PORT: Lazy<u16> = Lazy::new(|| {
    std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid 16-bit unsigned integer")
});
pub(crate) static HOST: Lazy<String> =
    Lazy::new(|| std::env::var("HOST").unwrap_or("127.0.0.1".to_string()));
pub(crate) static ENVIRONMENT: Lazy<String> =
    Lazy::new(|| std::env::var("ENVIRONMENT").unwrap_or("development".to_string()));
pub(crate) static FRONTEND_DIST: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("FRONTEND_DIST")
        .unwrap_or("./dist".to_string())
        .parse::<PathBuf>()
        .expect("Invalid path format")
});

// this is a separate thing of itself and is NOT related with PORT and HOST. this is strictly for production use when there are actual domains
pub(crate) static APP_DOMAIN: Lazy<String> =
    Lazy::new(|| std::env::var("DOMAIN").unwrap_or(format!("localhost:{}", *PORT)));

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

pub(crate) static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("DATABASE_URL")
        .unwrap_or("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
});
