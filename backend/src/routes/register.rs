use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::structs::APIResponse;
#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    password: String,
    confirm_password: String,
}

#[derive(Serialize)]
struct RegisterFormInputTarget {
    target_field: String,
}

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
