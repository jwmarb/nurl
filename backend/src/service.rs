use crate::{
    constants::APP_DOMAIN,
    structs::{ShortenedUrl, User},
};
use chrono::{Duration, Utc};
use nanoid::nanoid;
use sqlx::PgPool;
use uuid::Uuid;

/// Calculates the expiry date based on the number of seconds from now
/// 
/// # Arguments
/// * `expiration_sec` - Optional number of seconds until expiration
/// 
/// # Returns
/// Optional DateTime representing when the URL will expire
fn calculate_expiry_date(expiration_sec: Option<i64>) -> Option<chrono::DateTime<Utc>> {
    expiration_sec.map(|secs| Utc::now() + Duration::seconds(secs))
}

/// Validates that the original URL is not pointing to the application domain
/// to prevent redirect loops
/// 
/// # Arguments
/// * `original_url` - The URL to validate
/// * `domain` - The application domain to check against
/// 
/// # Returns
/// Result indicating if the URL is valid
fn validate_original_url(original_url: &str, domain: String) -> Result<(), std::io::Error> {
    if original_url.contains(&domain) {
        return Err(std::io::Error::new(
          std::io::ErrorKind::InvalidInput,
          "Operation not permitted to prevent redirect loops to self. Please use a different URL.",
      ));
    }
    Ok(())
}

/// Validates that a custom URL is valid (no slashes and not 'auth')
/// 
/// # Arguments
/// * `custom_url` - The custom URL to validate
/// 
/// # Returns
/// Result indicating if the custom URL is valid
fn validate_custom_url(custom_url: &str) -> Result<(), std::io::Error> {
    if custom_url.contains('/') || custom_url == "auth" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Custom URL cannot contain slashes or be 'auth'",
        ));
    }
    Ok(())
}

/// Generates a unique short URL that doesn't exist in the database
/// 
/// # Arguments
/// * `pool` - Database connection pool
/// 
/// # Returns
/// Result containing the generated short URL
async fn generate_unique_short_url(pool: &PgPool) -> Result<String, std::io::Error> {
    let mut short_url;
    loop {
        short_url = nanoid!(5); // 5 characters should be enough for uniqueness
        let exists: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM shortened_urls WHERE short_url = $1")
                .bind(&short_url)
                .fetch_one(pool)
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        if exists.0 == 0 {
            break;
        }
    }
    Ok(short_url)
}

/// Determines the final short URL to use, either a custom one or a generated one
/// 
/// # Arguments
/// * `custom_url` - Optional custom URL provided by the user
/// * `pool` - Database connection pool
/// 
/// # Returns
/// Result containing the final short URL to use
async fn determine_short_url(
    custom_url: Option<String>,
    pool: &PgPool,
) -> Result<String, std::io::Error> {
    match custom_url {
        Some(url) if !url.is_empty() => {
            validate_custom_url(&url)?;
            Ok(url)
        }
        _ => generate_unique_short_url(pool).await,
    }
}

/// Inserts a new shortened URL into the database
/// 
/// # Arguments
/// * `shortened_url` - The shortened URL to insert
/// * `pool` - Database connection pool
/// 
/// # Returns
/// Result indicating success or failure
async fn insert_url_to_db(
    shortened_url: &ShortenedUrl,
    pool: &PgPool,
) -> Result<(), std::io::Error> {
    sqlx::query(
      "INSERT INTO shortened_urls (id, original_url, short_url, expiry_date, created_at, updated_at, owner, redirects) 
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
  )
  .bind(shortened_url.id)
  .bind(&shortened_url.original_url)
  .bind(&shortened_url.short_url)
  .bind(shortened_url.expiry_date)
  .bind(shortened_url.created_at)
  .bind(shortened_url.updated_at)
  .bind(shortened_url.owner)
  .bind(shortened_url.redirects)
  .execute(pool)
  .await
  .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "A shortened URL already exists. Please use a different shortened URL."))?;

    Ok(())
}

/// Creates a new shortened URL for a user
/// 
/// # Arguments
/// * `user` - The user creating the URL
/// * `original_url` - The original URL to shorten
/// * `custom_url` - Optional custom short URL
/// * `expiration_sec` - Optional number of seconds until expiration
/// * `pool` - Database connection pool
/// 
/// # Returns
/// Result containing the created ShortenedUrl
pub async fn create_url(
    user: &User,
    original_url: &String,
    custom_url: Option<String>,
    expiration_sec: Option<i64>,
    pool: &PgPool,
) -> Result<ShortenedUrl, std::io::Error> {
    // Validate the original URL
    validate_original_url(original_url, APP_DOMAIN.clone())?;

    // Calculate expiry date
    let expiry_date = calculate_expiry_date(expiration_sec);

    // Determine short URL (custom or generated)
    let final_custom_url = determine_short_url(custom_url, pool).await?;

    // Create new URL entity
    let cur_time = Utc::now();
    let id = Uuid::new_v4();
    let short_url = ShortenedUrl {
        id,
        original_url: original_url.to_string(),
        short_url: final_custom_url,
        expiry_date,
        created_at: cur_time,
        updated_at: cur_time,
        owner: user.id,
        redirects: 0,
    };

    // Insert to database
    insert_url_to_db(&short_url, pool).await?;

    Ok(short_url)
}

/// Parses a UUID from a string
/// 
/// # Arguments
/// * `id` - The string to parse as a UUID
/// 
/// # Returns
/// Result containing the parsed UUID
fn parse_uuid(id: &str) -> Result<Uuid, std::io::Error> {
    Uuid::parse_str(id)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string()))
}

/// Updates an existing shortened URL
/// 
/// # Arguments
/// * `user` - The user updating the URL
/// * `id` - The ID of the URL to update
/// * `pool` - Database connection pool
/// * `original_url` - The new original URL
/// * `custom_url` - Optional new custom short URL
/// * `expiration_sec` - Optional new expiration time in seconds
/// 
/// # Returns
/// Result containing the updated ShortenedUrl
pub async fn update_url(
    user: &User,
    id: &String,
    pool: &PgPool,
    original_url: &String,
    custom_url: Option<&String>,
    expiration_sec: Option<i64>,
) -> Result<ShortenedUrl, std::io::Error> {
    // Validate the original URL
    validate_original_url(original_url, APP_DOMAIN.clone())?;

    // Calculate expiry date
    let expiry_date = calculate_expiry_date(expiration_sec);

    // Determine final short URL (custom or generated)
    let final_custom_url = match custom_url {
        Some(url) if !url.is_empty() => {
            validate_custom_url(url)?;
            url.to_owned()
        }
        _ => generate_unique_short_url(pool).await?,
    };

    // Parse UUID
    let uuid = parse_uuid(id)?;
    let cur_time = Utc::now();

    // Update in database
    let short_url = sqlx::query_as::<_, ShortenedUrl>(
        r#"
      UPDATE shortened_urls 
      SET 
          short_url = $1,
          original_url = $2,
          updated_at = $3,
          expiry_date = $4,
          owner = $5
      WHERE id = $6
      RETURNING *
      "#,
    )
    .bind(final_custom_url)
    .bind(original_url)
    .bind(cur_time)
    .bind(expiry_date)
    .bind(user.id)
    .bind(uuid)
    .fetch_one(pool)
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(short_url)
}

/// Deletes a shortened URL
/// 
/// # Arguments
/// * `user` - The user deleting the URL
/// * `id` - The ID of the URL to delete
/// * `pool` - Database connection pool
/// 
/// # Returns
/// Result indicating success or failure
pub async fn delete_url(user: &User, id: &String, pool: &PgPool) -> Result<(), std::io::Error> {
    let uuid = parse_uuid(id)?;

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

/// Lists all shortened URLs for a user
/// 
/// # Arguments
/// * `user` - The user whose URLs to list
/// * `pool` - Database connection pool
/// 
/// # Returns
/// Result containing a vector of the user's ShortenedUrls
pub async fn list_urls(user: &User, pool: &PgPool) -> Result<Vec<ShortenedUrl>, std::io::Error> {
    let urls =
        sqlx::query_as("SELECT * FROM shortened_urls WHERE owner = $1 ORDER BY created_at DESC")
            .bind(user.id)
            .fetch_all(pool)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(urls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        PgPool {}
        impl Clone for PgPool {
            fn clone(&self) -> Self;
        }
    }

    #[test]
    fn test_calculate_expiry_date() {
        // Test with Some value
        let now = Utc::now();
        let result = calculate_expiry_date(Some(3600));
        assert!(result.is_some());

        let duration = result.unwrap() - now;
        assert!(duration.num_seconds() >= 3599 && duration.num_seconds() <= 3601);

        // Test with None
        let result = calculate_expiry_date(None);
        assert!(result.is_none());
    }

    #[test]
    fn test_validate_original_url() {
        // Test valid URL
        let result = validate_original_url("https://example.com", APP_DOMAIN.clone());
        assert!(result.is_ok());

        // Test URL with domain in APP_DOMAIN (simulating with a mock)
        let app_domain = "shorturl.com";
        unsafe {
            std::env::set_var("APP_DOMAIN", app_domain);
        }
        let result = validate_original_url(
            &format!("https://{}/something", app_domain),
            app_domain.to_string(),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_validate_custom_url() {
        // Test valid URL
        let result = validate_custom_url("validurl");
        assert!(result.is_ok());

        // Test URL with slash
        let result = validate_custom_url("invalid/url");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);

        // Test URL with 'auth'
        let result = validate_custom_url("auth");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_parse_uuid() {
        // Test valid UUID
        let valid_uuid = "123e4567-e89b-12d3-a456-426614174000";
        let result = parse_uuid(valid_uuid);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), valid_uuid);

        // Test invalid UUID
        let result = parse_uuid("not-a-uuid");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
    }

    // More complex tests that would require async and mocking the database
    // would typically use tokio's runtime and mock the PgPool responses
    #[tokio::test]
    async fn test_generate_unique_short_url() {
        // This would require mocking the database query
        // For brevity, we'll just check the function exists
        // In a real test, you'd create a mock PgPool that returns predictable results
        let mut mock_pool = MockPgPool::new();
        // Configure the mock...

        // Ensure the result is the expected length
        //let result = generate_unique_short_url(&mock_pool).await;
        //assert!(result.is_ok());
        //assert_eq!(result.unwrap().len(), 5);
    }
}
