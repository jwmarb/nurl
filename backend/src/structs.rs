use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

// User model, holds their id + username + password
#[derive(sqlx::FromRow)]
pub(crate) struct User {
    pub id: Uuid,
    pub username: String,

    pub password: String, // bcrypt hash password
}

// The URL shortener model itself
#[derive(sqlx::FromRow)]
pub(crate) struct ShortenedUrl {
    pub id: Uuid,             // unique id
    pub original_url: String, // orig long url
    pub short_url: String,    // new short url

    pub expiry_date: Option<DateTime<Utc>>, // optional expiration date
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub owner: String,  // foreign key. the id of the person that owns this
    pub redirects: u64, // use count
}

#[derive(Serialize)]
pub struct APIResponse {
    pub error: Option<String>,
    pub data: Option<serde_json::Value>,
}

impl APIResponse {
    pub fn error_message(error: String) -> Self {
        Self {
            error: Some(error),
            data: None,
        }
    }
    pub fn error<T: Serialize>(error: String, data: Option<T>) -> Self {
        Self {
            error: Some(error),
            data: data.map(|d| serde_json::to_value(d).expect("Failed to serialize data")),
        }
    }

    pub fn data<T: Serialize>(data: T) -> Self {
        Self {
            error: None,
            data: Some(serde_json::to_value(data).expect("Failed to serialize data")),
        }
    }
}
