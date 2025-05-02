use crate::structs::{ShortenedUrl, User};
use chrono::{DateTime, Duration, Utc};
use nanoid::nanoid;
use sqlx::PgPool;
use uuid::Uuid;

// take in the user, orig url, custom url (we randomize if not provided),
// expiry (if not provided then no expiration)
pub async fn create_or_update_url(
    user: &User,
    original_url: &str,
    custom_url: Option<&str>,
    expiration_sec: Option<i64>,
    pool: &PgPool,
) -> Result<ShortenedUrl, std::io::Error> {
    let cur_time = Utc::now();

    // see if expiry provided, if so calculate the expiry date
    let expiry_date = expiration_sec.map(|secs| cur_time + Duration::seconds(secs));

    // Check if short url name provided. Otherwise generate a random one
    let final_custom_url = match custom_url {
        Some(url) if !url.is_empty() => {
            // Validate the custom URL, make sure it doesn't contain slashes or be 'auth'
            if url.contains('/') || url == "auth" {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Custom URL cannot contain slashes or be 'auth'",
                ));
            }
            url.to_owned()
        }
        _ => {
            // Generate a unique short URL using nanoid
            let mut short_url;
            loop {
                short_url = nanoid!(5); // 5 characters should be enough for uniqueness
                let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM shortened_urls WHERE short_url = $1")
                    .bind(&short_url)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
                
                if exists.0 == 0 {
                    break;
                }
            }
            short_url
        }
    };

    // Check if URL already exists for this user
    let existing_url: Option<ShortenedUrl> = sqlx::query_as(
        "SELECT * FROM shortened_urls WHERE original_url = $1 AND owner = $2"
    )
    .bind(original_url)
    .bind(user.id)
    .fetch_optional(pool)
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    if let Some(mut existing) = existing_url {
        // Update existing URL
        let short_url = existing.short_url.clone();
        let expiry_date = existing.expiry_date;
        let updated_at = existing.updated_at;
        let id = existing.id;

        sqlx::query(
            "UPDATE shortened_urls SET short_url = $1, expiry_date = $2, updated_at = $3 WHERE id = $4"
        )
        .bind(short_url)
        .bind(expiry_date)
        .bind(updated_at)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        // Update the existing struct with new values
        existing.short_url = final_custom_url;
        existing.expiry_date = expiry_date;
        existing.updated_at = cur_time;

        return Ok(existing);
    }

    // Create new URL
    let id = Uuid::new_v4();
    let short_url = ShortenedUrl {
        id,
        original_url: original_url.to_owned(),
        short_url: final_custom_url,
        expiry_date,
        created_at: cur_time,
        updated_at: cur_time,
        owner: user.id,
        redirects: 0,
    };

    // Clone values before moving them
    let id = short_url.id;
    let original_url = short_url.original_url.clone();
    let short_url_str = short_url.short_url.clone();
    let expiry_date = short_url.expiry_date;
    let created_at = short_url.created_at;
    let updated_at = short_url.updated_at;
    let owner = short_url.owner;
    let redirects = short_url.redirects;

    sqlx::query(
        "INSERT INTO shortened_urls (id, original_url, short_url, expiry_date, created_at, updated_at, owner, redirects) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(id)
    .bind(original_url)
    .bind(short_url_str)
    .bind(expiry_date)
    .bind(created_at)
    .bind(updated_at)
    .bind(owner)
    .bind(redirects)
    .execute(pool)
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(short_url)
}
// deletes a url (by id) for the user
pub async fn delete_url(user: &User, id: &str, pool: &PgPool) -> Result<(), std::io::Error> {
    let uuid = Uuid::parse_str(id)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string()))?;

    // Check ownership and delete
    let result = sqlx::query("DELETE FROM shortened_urls WHERE id = $1 AND owner = $2")
        .bind(uuid)
        .bind(user.id)
        .execute(pool)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "URL not found or you don't have permission to delete it",
        ));
    }

    Ok(())
}

// returns a list of the shortened urls for a given user
pub async fn list_urls(user: &User, pool: &PgPool) -> Result<Vec<ShortenedUrl>, std::io::Error> {
    let urls = sqlx::query_as("SELECT * FROM shortened_urls WHERE owner = $1 ORDER BY created_at DESC")
        .bind(user.id)
        .fetch_all(pool)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(urls)
}

// redirect shortened url to the actual one
pub async fn resolve_url(custom_url: &str, pool: &PgPool) -> Result<String, std::io::Error> {
    let url: Option<ShortenedUrl> = sqlx::query_as("SELECT * FROM shortened_urls WHERE short_url = $1")
        .bind(custom_url)
        .fetch_optional(pool)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let url = url.ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "URL not found")
    })?;

    // Check if URL has expired
    if let Some(expiry) = url.expiry_date {
        if Utc::now() > expiry {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "URL has expired",
            ));
        }
    }

    // Increment redirect count
    sqlx::query("UPDATE shortened_urls SET redirects = redirects + 1 WHERE id = $1")
        .bind(url.id)
        .execute(pool)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(url.original_url)
}
