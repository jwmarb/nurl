use actix_web::{delete, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    service::{create_or_update_url, delete_url},
    structs::{APIResponse, User},
};

#[derive(Deserialize)]
struct ShortenURLRequest {
    original_url: String,
    custom_path: Option<String>,
    expiration: Option<i64>,
}

#[post("/api/shorten")]
pub async fn shorten_url(
    body: web::Json<ShortenURLRequest>,
    pool: web::Data<PgPool>,
    username: web::ReqData<String>,
) -> impl Responder {
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&username.to_string())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(u) => u,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    match create_or_update_url(
        &user,
        &body.original_url,
        body.custom_path.clone(),
        body.expiration,
        pool.get_ref(),
    )
    .await
    {
        Ok(url) => HttpResponse::Ok().json(APIResponse::data(url)),
        Err(e) => {
            HttpResponse::InternalServerError().json(APIResponse::error_message(e.to_string()))
        }
    }
}

#[delete("/api/shorten/{id}")]
pub async fn delete_shortened_url(
    id: web::Path<String>,
    pool: web::Data<PgPool>,
    username: web::ReqData<String>,
) -> impl Responder {
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&username.to_string())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(u) => u,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let s = id.into_inner();
    match delete_url(&user, &s, pool.get_ref()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
