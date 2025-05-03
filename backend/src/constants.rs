use once_cell::sync::Lazy;
use rand::Rng;

use std::path::PathBuf;

/// The port number the server will listen on
/// Defaults to 8080 if not specified in environment variables
pub(crate) static PORT: Lazy<u16> = Lazy::new(|| {
    std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid 16-bit unsigned integer")
});

/// The host address the server will bind to
/// Defaults to 127.0.0.1 if not specified in environment variables
pub(crate) static HOST: Lazy<String> =
    Lazy::new(|| std::env::var("HOST").unwrap_or("127.0.0.1".to_string()));

/// The current environment (development/production)
/// Defaults to "development" if not specified in environment variables
pub(crate) static ENVIRONMENT: Lazy<String> =
    Lazy::new(|| std::env::var("ENVIRONMENT").unwrap_or("development".to_string()));

/// The path to the frontend distribution directory
/// Defaults to "./dist" if not specified in environment variables
pub(crate) static FRONTEND_DIST: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("FRONTEND_DIST")
        .unwrap_or("./dist".to_string())
        .parse::<PathBuf>()
        .expect("Invalid path format")
});

/// The domain name of the application
/// Used for URL validation and redirects
/// Defaults to "localhost:{PORT}" if not specified in environment variables
pub(crate) static APP_DOMAIN: Lazy<String> =
    Lazy::new(|| std::env::var("DOMAIN").unwrap_or(format!("localhost:{}", *PORT)));

/// The secret key used for JWT token generation
/// In development, uses a fixed key for convenience
/// In production, generates a random 32-character string if not specified
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

/// The string identifier for production environment
pub(crate) static PRODUCTION_ENV: &str = "production";

/// The database connection URL
/// Defaults to a local PostgreSQL instance if not specified in environment variables
pub(crate) static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("DATABASE_URL")
        .unwrap_or("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
});
