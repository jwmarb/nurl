use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user in the system
/// 
/// This struct is used to store user information in the database
/// and for authentication purposes
#[derive(Debug, sqlx::FromRow, Clone)]
pub(crate) struct User {
    /// Unique identifier for the user
    pub id: Uuid,
    /// Username used for login
    pub username: String,
    /// Bcrypt hashed password
    pub password: String, // bcrypt hash password
}

/// JWT claims used for authentication
/// 
/// This struct represents the data stored in JWT tokens
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    /// Username of the authenticated user
    pub username: String,
    /// Expiration timestamp of the token
    pub exp: usize,
}

/// Represents a shortened URL in the system
/// 
/// This struct is used to store and manage shortened URLs
#[derive(sqlx::FromRow, Serialize)]
pub(crate) struct ShortenedUrl {
    /// Unique identifier for the shortened URL
    pub id: Uuid,             // unique id
    /// The original long URL that was shortened
    pub original_url: String, // orig long url
    /// The shortened URL identifier
    pub short_url: String,    // new short url

    /// Optional date when the URL will expire
    pub expiry_date: Option<DateTime<Utc>>, // optional expiration date
    /// When the URL was created
    pub created_at: DateTime<Utc>,
    /// When the URL was last updated
    pub updated_at: DateTime<Utc>,

    /// ID of the user who owns this shortened URL
    pub owner: Uuid,    // foreign key. the id of the person that owns this
    /// Number of times this URL has been accessed
    pub redirects: i64, // use count
}

/// Standard API response format
/// 
/// This struct is used to standardize API responses across the application
#[derive(Serialize, Deserialize)]
pub struct APIResponse {
    /// Optional error message
    pub error: Option<String>,
    /// Optional response data
    pub data: Option<serde_json::Value>,
}

impl APIResponse {
    /// Creates an error response with just an error message
    /// 
    /// # Arguments
    /// * `error` - The error message to include
    pub fn error_message(error: String) -> Self {
        Self {
            error: Some(error),
            data: None,
        }
    }

    /// Creates an error response with both an error message and data
    /// 
    /// # Arguments
    /// * `error` - The error message to include
    /// * `data` - Optional data to include with the error
    pub fn error<T: Serialize>(error: String, data: Option<T>) -> Self {
        Self {
            error: Some(error),
            data: data.map(|d| serde_json::to_value(d).expect("Failed to serialize data")),
        }
    }

    /// Creates a successful response with data
    /// 
    /// # Arguments
    /// * `data` - The data to include in the response
    pub fn data<T: Serialize>(data: T) -> Self {
        Self {
            error: None,
            data: Some(serde_json::to_value(data).expect("Failed to serialize data")),
        }
    }
}
