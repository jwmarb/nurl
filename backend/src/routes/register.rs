use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::structs::APIResponse;

/// Registration form data structure
#[derive(Deserialize, Serialize)]
struct RegisterForm {
    /// Username for the new account
    username: String,
    /// Password for the new account
    password: String,
    /// Password confirmation to ensure correct entry
    confirm_password: String,
}

/// Structure for identifying which form field caused an error
#[derive(Serialize, Deserialize)]
struct RegisterFormInputTarget {
    /// The name of the form field that failed validation
    target_field: String,
}

/// Handles user registration
/// 
/// This endpoint:
/// 1. Validates the registration form data
/// 2. Checks if the username is already taken
/// 3. Hashes the password
/// 4. Creates the new user account
/// 
/// # Arguments
/// * `form` - The registration form data
/// * `pool` - Database connection pool
/// 
/// # Returns
/// HTTP response:
/// - 200 OK if registration is successful
/// - 400 Bad Request if validation fails (with specific error messages)
/// - 500 Internal Server Error if database operations fail
#[post("/register")]
async fn register(form: web::Json<RegisterForm>, pool: web::Data<PgPool>) -> impl Responder {
    let form = form.into_inner();

    if form.username.is_empty() {
        return HttpResponse::BadRequest().json(APIResponse::error(
            "Username cannot be empty".to_string(),
            Some(RegisterFormInputTarget {
                target_field: "username".to_string(),
            }),
        ));
    }

    if form.username.len() < 3 {
        return HttpResponse::BadRequest().json(APIResponse::error(
            "Username must be at least 3 characters long".to_string(),
            Some(RegisterFormInputTarget {
                target_field: "username".to_string(),
            }),
        ));
    }

    if form.password.is_empty() {
        return HttpResponse::BadRequest().json(APIResponse::error(
            "Password cannot be empty".to_string(),
            Some(RegisterFormInputTarget {
                target_field: "password".to_string(),
            }),
        ));
    }

    if form.password.len() < 6 {
        return HttpResponse::BadRequest().json(APIResponse::error(
            "Password must be at least 6 characters long".to_string(),
            Some(RegisterFormInputTarget {
                target_field: "password".to_string(),
            }),
        ));
    }

    if form.password != form.confirm_password {
        return HttpResponse::BadRequest().json(APIResponse::error(
            "Passwords do not match".to_string(),
            Some(RegisterFormInputTarget {
                target_field: "confirm_password".to_string(),
            }),
        ));
    }

    let exists: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = $1")
        .bind(&form.username)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => row,
        Err(_) => {
            return HttpResponse::InternalServerError().json(APIResponse::error_message(
                "Could not check if username exists".to_string(),
            ))
        }
    };

    if exists.0 > 0 {
        return HttpResponse::BadRequest().json(APIResponse::error(
            "Username already exists".to_string(),
            Some(RegisterFormInputTarget {
                target_field: "username".to_string(),
            }),
        ));
    }

    let hashed = match hash(&form.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return HttpResponse::InternalServerError().json(APIResponse::error_message(
                "Could not hash password".to_string(),
            ))
        }
    };

    let result = sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id")
        .bind(&form.username)
        .bind(&hashed)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().json(APIResponse::error_message(
            "Could not create user".to_string(),
        )),
    }
}

/// Test module for registration endpoint
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::init_test_db;
    use actix_web::{http::StatusCode, test, web, App};
    use uuid::Uuid;

    /// Tests successful user registration
    /// 
    /// Verifies that:
    /// 1. The registration request succeeds
    /// 2. The user is created in the database
    /// 3. The password is properly hashed
    #[actix_rt::test]
    async fn test_register_success() {
        let pool = init_test_db().await;

        // Create a unique username to avoid conflicts
        let unique_username = format!(
            "testuser_{}",
            Uuid::new_v4()
                .to_string()
                .chars()
                .take(8)
                .collect::<String>()
        );
        let password = "password123";

        // Create test app with the handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register),
        )
        .await;

        // Create the registration form
        let form = RegisterForm {
            username: unique_username.clone(),
            password: password.to_string(),
            confirm_password: password.to_string(),
        };

        // Send test request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&form)
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Assert the response
        assert_eq!(resp.status(), StatusCode::OK);

        // Verify the user was created
        let user_exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = $1")
            .bind(&unique_username)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch user");

        assert_eq!(user_exists.0, 1);

        // Clean up
        sqlx::query("DELETE FROM users WHERE username = $1")
            .bind(&unique_username)
            .execute(&pool)
            .await
            .expect("Failed to delete test user");
    }

    /// Tests registration with empty username
    /// 
    /// Verifies that:
    /// 1. The request fails with 400 Bad Request
    /// 2. The correct error message is returned
    /// 3. The target field is correctly identified
    #[actix_rt::test]
    async fn test_register_empty_username() {
        let pool = init_test_db().await;

        // Create test app with the handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register),
        )
        .await;

        // Create the registration form with empty username
        let form = RegisterForm {
            username: "".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
        };

        // Send test request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&form)
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Assert the response
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        // Check the response body contains the expected error
        let body = test::read_body(resp).await;
        let response: APIResponse =
            serde_json::from_slice(&body).expect("Failed to parse response");

        assert_eq!(response.error, Some("Username cannot be empty".to_string()));

        if let Some(data) = response.data {
            let target = serde_json::from_value::<RegisterFormInputTarget>(data)
                .expect("Failed to deserialize target field");
            assert_eq!(target.target_field, "username");
        } else {
            panic!("Expected target field in response");
        }
    }

    /// Tests registration with username shorter than 3 characters
    /// 
    /// Verifies that:
    /// 1. The request fails with 400 Bad Request
    /// 2. The correct error message is returned
    /// 3. The target field is correctly identified
    #[actix_rt::test]
    async fn test_register_short_username() {
        let pool = init_test_db().await;

        // Create test app with the handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register),
        )
        .await;

        // Create the registration form with short username
        let form = RegisterForm {
            username: "ab".to_string(), // Less than 3 characters
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
        };

        // Send test request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&form)
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Assert the response
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        // Check the response body contains the expected error
        let body = test::read_body(resp).await;
        let response: APIResponse =
            serde_json::from_slice(&body).expect("Failed to parse response");

        assert_eq!(
            response.error,
            Some("Username must be at least 3 characters long".to_string())
        );

        if let Some(data) = response.data {
            let target = serde_json::from_value::<RegisterFormInputTarget>(data)
                .expect("Failed to deserialize target field");
            assert_eq!(target.target_field, "username");
        } else {
            panic!("Expected target field in response");
        }
    }

    /// Tests registration with empty password
    /// 
    /// Verifies that:
    /// 1. The request fails with 400 Bad Request
    /// 2. The correct error message is returned
    /// 3. The target field is correctly identified
    #[actix_rt::test]
    async fn test_register_empty_password() {
        let pool = init_test_db().await;

        // Create test app with the handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register),
        )
        .await;

        // Create the registration form with empty password
        let form = RegisterForm {
            username: "validuser".to_string(),
            password: "".to_string(), // Empty password
            confirm_password: "".to_string(),
        };

        // Send test request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&form)
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Assert the response
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        // Check the response body contains the expected error
        let body = test::read_body(resp).await;
        let response: APIResponse =
            serde_json::from_slice(&body).expect("Failed to parse response");

        assert_eq!(response.error, Some("Password cannot be empty".to_string()));

        if let Some(data) = response.data {
            let target = serde_json::from_value::<RegisterFormInputTarget>(data)
                .expect("Failed to deserialize target field");
            assert_eq!(target.target_field, "password");
        } else {
            panic!("Expected target field in response");
        }
    }

    /// Tests registration with password shorter than 6 characters
    /// 
    /// Verifies that:
    /// 1. The request fails with 400 Bad Request
    /// 2. The correct error message is returned
    /// 3. The target field is correctly identified
    #[actix_rt::test]
    async fn test_register_short_password() {
        let pool = init_test_db().await;

        // Create test app with the handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register),
        )
        .await;

        // Create the registration form with short password
        let form = RegisterForm {
            username: "validuser".to_string(),
            password: "12345".to_string(), // Less than 6 characters
            confirm_password: "12345".to_string(),
        };

        // Send test request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&form)
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Assert the response
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        // Check the response body contains the expected error
        let body = test::read_body(resp).await;
        let response: APIResponse =
            serde_json::from_slice(&body).expect("Failed to parse response");

        assert_eq!(
            response.error,
            Some("Password must be at least 6 characters long".to_string())
        );

        if let Some(data) = response.data {
            let target = serde_json::from_value::<RegisterFormInputTarget>(data)
                .expect("Failed to deserialize target field");
            assert_eq!(target.target_field, "password");
        } else {
            panic!("Expected target field in response");
        }
    }

    /// Tests registration with mismatched passwords
    /// 
    /// Verifies that:
    /// 1. The request fails with 400 Bad Request
    /// 2. The correct error message is returned
    /// 3. The target field is correctly identified
    #[actix_rt::test]
    async fn test_register_password_mismatch() {
        let pool = init_test_db().await;

        // Create test app with the handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register),
        )
        .await;

        // Create the registration form with mismatched passwords
        let form = RegisterForm {
            username: "validuser".to_string(),
            password: "password123".to_string(),
            confirm_password: "password456".to_string(), // Different password
        };

        // Send test request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&form)
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Assert the response
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        // Check the response body contains the expected error
        let body = test::read_body(resp).await;
        let response: APIResponse =
            serde_json::from_slice(&body).expect("Failed to parse response");

        assert_eq!(response.error, Some("Passwords do not match".to_string()));

        if let Some(data) = response.data {
            let target = serde_json::from_value::<RegisterFormInputTarget>(data)
                .expect("Failed to deserialize target field");
            assert_eq!(target.target_field, "confirm_password");
        } else {
            panic!("Expected target field in response");
        }
    }

    /// Tests registration with duplicate username
    /// 
    /// Verifies that:
    /// 1. The request fails with 400 Bad Request
    /// 2. The correct error message is returned
    /// 3. The target field is correctly identified
    #[actix_rt::test]
    async fn test_register_duplicate_username() {
        let pool = init_test_db().await;

        // Create a unique username for this test
        let unique_username = format!(
            "testuser_{}",
            Uuid::new_v4()
                .to_string()
                .chars()
                .take(8)
                .collect::<String>()
        );
        let password = "password123";

        // First, create a user
        let hashed = hash(password, DEFAULT_COST).expect("Failed to hash password");
        sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
            .bind(&unique_username)
            .bind(&hashed)
            .execute(&pool)
            .await
            .expect("Failed to insert test user");

        // Create test app with the handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register),
        )
        .await;

        // Try to register with the same username
        let form = RegisterForm {
            username: unique_username.clone(),
            password: "newpassword123".to_string(),
            confirm_password: "newpassword123".to_string(),
        };

        // Send test request
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&form)
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Assert the response
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        // Check the response body contains the expected error
        let body = test::read_body(resp).await;
        let response: APIResponse =
            serde_json::from_slice(&body).expect("Failed to parse response");

        assert_eq!(response.error, Some("Username already exists".to_string()));

        if let Some(data) = response.data {
            let target = serde_json::from_value::<RegisterFormInputTarget>(data)
                .expect("Failed to deserialize target field");
            assert_eq!(target.target_field, "username");
        } else {
            panic!("Expected target field in response");
        }

        // Clean up
        sqlx::query("DELETE FROM users WHERE username = $1")
            .bind(&unique_username)
            .execute(&pool)
            .await
            .expect("Failed to delete test user");
    }
}
