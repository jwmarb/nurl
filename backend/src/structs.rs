use chrono::{DateTime, Utc};

// User model, holds their id + username + password
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,

    pub password: String, // bcrypt hash password
}

// The URL shortener model itself
#[derive(Debug, Clone)]
pub struct ShortenedUrl {
    pub id: String, // unique id
    pub original_url: String, // orig long url
    pub short_url: String, // new short url

    pub expiry_date: Option<DateTime<Utc>>, // optional expiration date
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub owner: String, // the id of the person that owns this
    pub redirects: u64, // use count
}

/// Errors
#[derive(Debug)]
pub enum ShortenError {
    NotFound,
    Unauthorized,
    InvalidInput(String),
    DatabaseError(String),
}