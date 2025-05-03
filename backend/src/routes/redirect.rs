use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::structs::ShortenedUrl;

/// Redirects a short URL to its original destination
/// 
/// This endpoint:
/// 1. Looks up the short URL in the database
/// 2. Checks if the URL has expired
/// 3. Increments the redirect counter
/// 4. Returns a 307 Temporary Redirect to the original URL
/// 
/// # Arguments
/// * `pool` - Database connection pool
/// * `short_path` - The short URL path to redirect from
/// 
/// # Returns
/// HTTP response:
/// - 307 Temporary Redirect with Location header if URL is valid
/// - 404 Not Found if URL doesn't exist or has expired
/// - 500 Internal Server Error if database update fails
#[get("/{short_path}")]
pub async fn redirect_to_original_url(
    pool: web::Data<PgPool>,
    short_path: web::Path<String>,
) -> impl Responder {
    let shortened_url = match sqlx::query_as::<_, ShortenedUrl>(
        "SELECT * FROM shortened_urls WHERE short_url = $1",
    )
    .bind(&short_path.to_string())
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(u) => u,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    if let Some(expiry_date) = shortened_url.expiry_date {
        if expiry_date < chrono::Utc::now() {
            return HttpResponse::NotFound().finish();
        }
    }

    match sqlx::query("UPDATE shortened_urls SET redirects = redirects + 1 WHERE id = $1")
        .bind(shortened_url.id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
    {
        Ok(_) => (),
        Err(e) => return e,
    };

    let original_url = shortened_url.original_url;
    println!("Redirecting to: {}", original_url);

    HttpResponse::TemporaryRedirect()
        .append_header(("Location", original_url))
        .finish()
}

/// Test module for the redirect endpoint
#[cfg(test)]
mod tests {

    use crate::utils::{get_test_user, init_test_db};

    use super::*;
    use actix_web::{http::StatusCode, test, web, App};
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    /// Tests successful redirection of a valid short URL
    /// 
    /// This test:
    /// 1. Creates a test short URL
    /// 2. Makes a request to the redirect endpoint
    /// 3. Verifies the redirect response
    /// 4. Checks that the redirect counter was incremented
    #[actix_rt::test]
    async fn test_redirect_success() {
        let pool = init_test_db().await;
        let test_user = get_test_user(&pool).await;

        // Set up test data with a unique short path
        let test_id = Uuid::new_v4();
        let short_path = format!(
            "test_{}",
            Uuid::new_v4()
                .to_string()
                .chars()
                .take(6)
                .collect::<String>()
        );
        let original_url = "https://example.com";

        // Insert test data into the test database
        sqlx::query(
            "INSERT INTO shortened_urls (id, short_url, original_url, redirects, created_at, updated_at, owner) 
           VALUES ($1, $2, $3, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4)",
        )
        .bind(test_id)
        .bind(&short_path)
        .bind(original_url)
        .bind(test_user.id)
        .execute(&pool)
        .await
        .expect("Failed to insert test data");

        // Create test app with the handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(redirect_to_original_url),
        )
        .await;

        // Send test request
        let req = test::TestRequest::get()
            .uri(&format!("/{}", short_path))
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Assert the response
        assert_eq!(resp.status(), StatusCode::TEMPORARY_REDIRECT);

        let location = resp.headers().get("Location").unwrap();
        assert_eq!(location, original_url);

        // Verify the redirect count was incremented
        let updated_url =
            sqlx::query_as::<_, ShortenedUrl>("SELECT * FROM shortened_urls WHERE id = $1")
                .bind(test_id)
                .fetch_one(&pool)
                .await
                .expect("Failed to fetch updated url");

        assert_eq!(updated_url.redirects, 1);

        // Clean up the specific test data first to avoid foreign key constraint issues
        sqlx::query("DELETE FROM shortened_urls WHERE id = $1")
            .bind(test_id)
            .execute(&pool)
            .await
            .expect("Failed to delete test URL");
    }

    /// Tests handling of non-existent short URLs
    /// 
    /// This test verifies that the endpoint returns a 404 response
    /// when the short URL doesn't exist in the database
    #[actix_rt::test]
    async fn test_redirect_not_found() {
        let pool = init_test_db().await;

        // Create test app with the handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(redirect_to_original_url),
        )
        .await;

        // Send test request with a non-existent short URL
        let req = test::TestRequest::get()
            .uri("/nonexistent-test-path")
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Assert not found response
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    /// Tests handling of expired short URLs
    /// 
    /// This test:
    /// 1. Creates a short URL with an expired date
    /// 2. Makes a request to the redirect endpoint
    /// 3. Verifies that a 404 response is returned
    #[actix_rt::test]
    async fn test_redirect_expired() {
        let pool = init_test_db().await;
        let test_user = get_test_user(&pool).await;

        // Set up test data with an expired link and unique short path
        let test_id = Uuid::new_v4();
        let short_path = format!(
            "expired_{}",
            Uuid::new_v4()
                .to_string()
                .chars()
                .take(6)
                .collect::<String>()
        );
        let original_url = "https://example.com/expired";
        let expired_date = Utc::now() - Duration::days(1); // 1 day ago

        // Insert test data into the test database
        sqlx::query(
          "INSERT INTO shortened_urls (id, short_url, original_url, redirects, created_at, updated_at, expiry_date, owner) 
           VALUES ($1, $2, $3, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4, $5)"
        )
        .bind(test_id)
        .bind(&short_path)
        .bind(original_url)
        .bind(expired_date)
        .bind(test_user.id)
        .execute(&pool)
        .await
        .expect("Failed to insert test data");

        // Create test app with the handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(redirect_to_original_url),
        )
        .await;

        // Send test request
        let req = test::TestRequest::get()
            .uri(&format!("/{}", short_path))
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Assert not found response for expired URL
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        // Clean up the specific test data first to avoid foreign key constraint issues
        sqlx::query("DELETE FROM shortened_urls WHERE id = $1")
            .bind(test_id)
            .execute(&pool)
            .await
            .expect("Failed to delete test URL");
    }
}
