use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize)]
struct Claims {
    pub username: String,
    pub exp: usize,
}

pub async fn login_user(form: LoginForm, db: &PgPool) -> Result<TokenResponse, String> {
    let user = sqlx::query!(
        "SELECT password FROM users WHERE username = $1",
        form.username
    )
    .fetch_optional(db)
    .await
    .map_err(|_| "DB error")?;

    if let Some(record) = user {
        let password_matches = verify(&form.password, &record.password)
            .map_err(|_| "Bcrypt error")?;

        if password_matches {
            let claims = Claims {
                username: form.username,
                exp: chrono::Utc::now().timestamp() as usize + 3600,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("your-secret".as_ref()),
            )
            .map_err(|_| "JWT error")?;

            return Ok(TokenResponse { token });
        }
    }

    Err("Invalid username or password".to_string())
}
    

/* 
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize)]
struct Claims {
    pub username: String,
    pub exp: usize,
}

// Traits for abstraction
pub trait UserRepository {
    fn get_password_hash(&self, username: &str) -> Result<Option<String>, String>;
}

pub trait TokenService {
    fn create_token(&self, username: &str) -> Result<String, String>;
}

// The login logic
pub fn login_user(
    username: &str,
    password: &str,
    repo: &dyn UserRepository,
    token_service: &dyn TokenService,
) -> Result<TokenResponse, String> {
    let hash_opt = repo.get_password_hash(username)?;

    if let Some(hash) = hash_opt {
        if verify(password, &hash).map_err(|_| "Bcrypt error")? {
            let token = token_service.create_token(username)?;
            return Ok(TokenResponse { token });
        }
    }

    Err("Invalid username or password".to_string())
}

// ========== Tests ==========
#[cfg(test)]
mod tests {
    use super::*;

    struct MockUserRepository;

    impl UserRepository for MockUserRepository {
        fn get_password_hash(&self, username: &str) -> Result<Option<String>, String> {
            if username == "testuser" {
                Ok(Some(hash("password123", 4).unwrap()))
            } else {
                Ok(None)
            }
        }
    }

    struct MockTokenService;

    impl TokenService for MockTokenService {
        fn create_token(&self, _username: &str) -> Result<String, String> {
            Ok("fake-jwt-token".to_string())
        }
    }

    #[test]
    fn test_login_success() {
        let repo = MockUserRepository;
        let token_service = MockTokenService;

        let result = login_user("testuser", "password123", &repo, &token_service);
        assert_eq!(
            result.unwrap(),
            TokenResponse {
                token: "fake-jwt-token".to_string()
            }
        );
    }

    #[test]
    fn test_login_wrong_password() {
        let repo = MockUserRepository;
        let token_service = MockTokenService;

        let result = login_user("testuser", "wrongpass", &repo, &token_service);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid username or password");
    }

    #[test]
    fn test_login_user_not_found() {
        let repo = MockUserRepository;
        let token_service = MockTokenService;

        let result = login_user("unknown", "anything", &repo, &token_service);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid username or password");
    }
}
*/