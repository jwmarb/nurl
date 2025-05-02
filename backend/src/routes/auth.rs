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

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[post("/auth")]
pub async fn login(form: web::Json<LoginForm>, db: web::Data<PgPool>) -> impl Responder {
    let row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&form.username)
        .fetch_one(db.get_ref())
        .await;

    let user = match row {
        Err(_) => {
            println!("Could not find user \"{}\"", form.username);
            return HttpResponse::UnprocessableEntity().json(APIResponse::error_message(
                "Invalid username/password".to_string(),
            ));
        }
        Ok(u) => u,
    };

    let password_matches = match verify(&form.password, &user.password) {
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

    let mut expiration = Utc::now() + Duration::hours(1);

    if form.remember_me {
        expiration = Utc::now() + Duration::days(30);
    }

    let jwt = encode(
        &Header::default(),
        &Claims {
            username: form.username.clone(),
            exp: expiration.timestamp() as usize,
        },
        &EncodingKey::from_secret((*NURL_SECRET).as_bytes()),
    )
    .unwrap();

    HttpResponse::Ok().json(APIResponse::data(TokenResponse { token: jwt }))
}

#[get("/auth")]
async fn is_authenticated(req: HttpRequest) -> impl Responder {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                let decoded = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(NURL_SECRET.as_bytes()),
                    &Validation::default(),
                );

                return match decoded {
                    Ok(_) => HttpResponse::Ok().json(APIResponse::data("authenticated")),
                    Err(_) => HttpResponse::Unauthorized()
                        .json(APIResponse::error_message("Invalid token".to_string())),
                };
            }
        }
    }
    HttpResponse::Unauthorized().json(APIResponse::error_message("Not authenticated".to_string()))
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
