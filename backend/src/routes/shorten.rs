use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    service::{create_url, delete_url, list_urls, update_url},
    structs::{APIResponse, User},
};

#[derive(Deserialize)]
struct ShortenURLRequest {
    original_url: String,
    custom_path: Option<String>,
    expiration: Option<i64>,
}

#[derive(Deserialize)]
struct UpdateURLRequest {
    id: String,
    original_url: String,
    custom_path: Option<String>,
    expiration: Option<i64>,
}

#[post("/shorten")]
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
    match create_url(
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

#[delete("/shorten/{id}")]
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
        Err(e) => {
            HttpResponse::InternalServerError().json(APIResponse::error_message(e.to_string()))
        }
    }
}

#[get("/shorten")]
pub async fn get_shortened_urls(
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

    match list_urls(&user, pool.get_ref()).await {
        Ok(urls) => HttpResponse::Ok().json(APIResponse::data(urls)),
        Err(e) => {
            HttpResponse::InternalServerError().json(APIResponse::error_message(e.to_string()))
        }
    }
}

#[put("/shorten")]
pub async fn update_shortened_url(
    pool: web::Data<PgPool>,
    username: web::ReqData<String>,
    url_data: web::Json<UpdateURLRequest>,
) -> impl Responder {
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&username.to_string())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(u) => u,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    match update_url(
        &user,
        &url_data.id,
        pool.get_ref(),
        &url_data.original_url,
        url_data.custom_path.as_ref(),
        url_data.expiration,
    )
    .await
    {
        Ok(url) => HttpResponse::Ok().json(APIResponse::data(url)),
        Err(e) => {
            HttpResponse::InternalServerError().json(APIResponse::error_message(e.to_string()))
        }
    }
}
