use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};

use serde::Deserialize;

#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    password: String,
    confirmPassword: String,
}


/* 
#[post("/api/register")]
async fn register(req: web::) -> impl Responder {
    HttpResponse::Ok().body("I'm healthy!")
}
    */


async fn register_user(
    form: web::Json<RegisterForm>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let form = form.into_inner();

    if form.password != form.confirmPassword {
        return HttpResponse::BadRequest().body("Passwords do not match");
    }

    let exists: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = $1")
        .bind(&form.username)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if exists.0 > 0 {
        return HttpResponse::BadRequest().body("Username already exists");
    }

    let hashed = match hash(&form.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let result = sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
        .bind(&form.username)
        .bind(&hashed)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User created"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
