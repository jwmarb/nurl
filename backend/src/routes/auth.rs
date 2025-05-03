use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    constants::NURL_SECRET,
    structs::{APIResponse, Claims, User},
};

#[derive(Deserialize, Clone)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

/// Finds a user by username in the database
async fn find_user(username: &str, db: &PgPool) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(db)
        .await
}

/// Validates user password
fn validate_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

/// Generates a JWT token for the user
fn generate_token(
    username: &str,
    remember_me: bool,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = if remember_me {
        Utc::now() + Duration::days(30)
    } else {
        Utc::now() + Duration::hours(1)
    };

    encode(
        &Header::default(),
        &Claims {
            username: username.to_string(),
            exp: expiration.timestamp() as usize,
        },
        &EncodingKey::from_secret((*NURL_SECRET).as_bytes()),
    )
}

/// Validates a token and returns the claims if valid
fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(NURL_SECRET.as_bytes()),
        &Validation::default(),
    )?;

    Ok(decoded.claims)
}

/// Extract token from Authorization header
fn extract_token_from_header(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .filter(|auth_str| auth_str.starts_with("Bearer "))
        .map(|auth_str| auth_str[7..].to_string())
}

#[post("/auth")]
pub async fn login(form: web::Json<LoginForm>, db: web::Data<PgPool>) -> impl Responder {
    // Find user in database
    let user = match find_user(&form.username, db.get_ref()).await {
        Err(_) => {
            println!("Could not find user \"{}\"", form.username);
            return HttpResponse::UnprocessableEntity().json(APIResponse::error_message(
                "Invalid username/password".to_string(),
            ));
        }
        Ok(u) => u,
    };

    // Validate password
    let password_matches = match validate_password(&form.password, &user.password) {
        Err(_) => {
            return HttpResponse::InternalServerError().json(APIResponse::error_message(
                "Could not verify password".to_string(),
            ));
        }
        Ok(m) => m,
    };

    if !password_matches {
        println!("Passwords do not match for user \"{}\"", form.username);
        return HttpResponse::UnprocessableEntity().json(APIResponse::error_message(
            "Invalid username/password".to_string(),
        ));
    }

    // Generate token
    let jwt = match generate_token(&form.username, form.remember_me) {
        Ok(token) => token,
        Err(_) => {
            return HttpResponse::InternalServerError().json(APIResponse::error_message(
                "Failed to generate token".to_string(),
            ));
        }
    };

    HttpResponse::Ok().json(APIResponse::data(TokenResponse { token: jwt }))
}

#[get("/auth")]
async fn is_authenticated(req: HttpRequest) -> impl Responder {
    // Extract token from headers
    let token = match extract_token_from_header(&req) {
        Some(token) => token,
        None => {
            return HttpResponse::Unauthorized()
                .json(APIResponse::error_message("Not authenticated".to_string()));
        }
    };

    // Validate token
    match validate_token(&token) {
        Ok(_) => HttpResponse::Ok().json(APIResponse::data("authenticated")),
        Err(_) => HttpResponse::Unauthorized()
            .json(APIResponse::error_message("Invalid token".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::Claims;
    use actix_web::test::TestRequest;
    use bcrypt::hash;
    use chrono::Utc;
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

    // Token validation tests
    #[test]
    fn test_extract_token_from_header() {
        // Create a mock request with an Authorization header
        let req = TestRequest::default()
            .insert_header(("Authorization", "Bearer test_token"))
            .to_http_request();

        // Test valid token extraction
        let token = extract_token_from_header(&req);
        assert_eq!(token, Some("test_token".to_string()));

        // Test without Bearer prefix
        let req = TestRequest::default()
            .insert_header(("Authorization", "NotBearer test_token"))
            .to_http_request();
        let token = extract_token_from_header(&req);
        assert_eq!(token, None);

        // Test without header
        let req = TestRequest::default().to_http_request();
        let token = extract_token_from_header(&req);
        assert_eq!(token, None);
    }

    #[test]
    fn test_validate_password() {
        // Generate a password hash
        let password = "test_password";
        let hashed = hash(password, 4).unwrap();

        // Valid password
        let result = validate_password(password, &hashed);
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Invalid password
        let result = validate_password("wrong_password", &hashed);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_generate_token() {
        let username = "test_user";

        // Test token with remember_me = false (1 hour)
        let token = generate_token(username, false).unwrap();
        let claims = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(NURL_SECRET.as_bytes()),
            &Validation::default(),
        )
        .unwrap()
        .claims;

        assert_eq!(claims.username, username);
        // Verify expiration is roughly 1 hour in the future (with small margin for test execution time)
        let now = Utc::now().timestamp() as usize;
        let hour_from_now = now + 3600;
        assert!(claims.exp > now);
        assert!(claims.exp <= hour_from_now + 5); // Allow 5 seconds margin

        // Test token with remember_me = true (30 days)
        let token = generate_token(username, true).unwrap();
        let claims = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(NURL_SECRET.as_bytes()),
            &Validation::default(),
        )
        .unwrap()
        .claims;

        assert_eq!(claims.username, username);
        // Verify expiration is roughly 30 days in the future
        let now = Utc::now().timestamp() as usize;
        let month_from_now = now + (30 * 24 * 3600);
        assert!(claims.exp > now);
        assert!(claims.exp <= month_from_now + 5); // Allow 5 seconds margin
    }

    #[test]
    fn test_validate_token() {
        let username = "test_user";
        let expiration = Utc::now() + Duration::hours(1);

        // Generate a valid token
        let valid_token = encode(
            &Header::default(),
            &Claims {
                username: username.to_string(),
                exp: expiration.timestamp() as usize,
            },
            &EncodingKey::from_secret((*NURL_SECRET).as_bytes()),
        )
        .unwrap();

        // Test valid token
        let result = validate_token(&valid_token);
        assert!(result.is_ok());
        let claims = result.unwrap();
        assert_eq!(claims.username, username);

        // Test expired token
        let expired_time = Utc::now() - Duration::hours(1);
        let expired_token = encode(
            &Header::default(),
            &Claims {
                username: username.to_string(),
                exp: expired_time.timestamp() as usize,
            },
            &EncodingKey::from_secret((*NURL_SECRET).as_bytes()),
        )
        .unwrap();

        let result = validate_token(&expired_token);
        assert!(result.is_err());

        // Test invalid token
        let result = validate_token("invalid.token.string");
        assert!(result.is_err());
    }
}
