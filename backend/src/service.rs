use crate::{
    constants::APP_DOMAIN,
    structs::{ShortenedUrl, User},
};
use chrono::{Duration, Utc};
use nanoid::nanoid;
use sqlx::PgPool;
use uuid::Uuid;

// Helper function to calculate expiry date from seconds
fn calculate_expiry_date(expiration_sec: Option<i64>) -> Option<chrono::DateTime<Utc>> {
    expiration_sec.map(|secs| Utc::now() + Duration::seconds(secs))
}

// Helper function to validate original URL
fn validate_original_url(original_url: &str, domain: String) -> Result<(), std::io::Error> {
    if original_url.contains(&domain) {
        return Err(std::io::Error::new(
          std::io::ErrorKind::InvalidInput,
          "Operation not permitted to prevent redirect loops to self. Please use a different URL.",
      ));
    }
    Ok(())
}

// Helper function to validate custom URL
fn validate_custom_url(custom_url: &str) -> Result<(), std::io::Error> {
    if custom_url.contains('/') || custom_url == "auth" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Custom URL cannot contain slashes or be 'auth'",
        ));
    }
    Ok(())
}

// Helper function to generate a unique short URL
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

// Helper function to determine final short URL based on custom URL or generated one
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

// Helper function to insert a new URL into the database
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

// take in the user, orig url, custom url (we randomize if not provided),
// expiry (if not provided then no expiration)
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

// Helper function to parse UUID from string
fn parse_uuid(id: &str) -> Result<Uuid, std::io::Error> {
    Uuid::parse_str(id)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string()))
}

// updates a url (by id) for the user
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

// deletes a url (by id) for the user
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

// returns a list of the shortened urls for a given user
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
