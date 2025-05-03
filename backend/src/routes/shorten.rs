use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    service::{create_url, delete_url, list_urls, update_url},
    structs::{APIResponse, User},
};

/// Request body for creating a new shortened URL
#[derive(Deserialize)]
struct ShortenURLRequest {
    /// The original URL to be shortened
    original_url: String,
    /// Optional custom path for the shortened URL
    custom_path: Option<String>,
    /// Optional expiration time in seconds
    expiration: Option<i64>,
}

/// Request body for updating an existing shortened URL
#[derive(Deserialize)]
struct UpdateURLRequest {
    /// The ID of the URL to update
    id: String,
    /// The new original URL
    original_url: String,
    /// Optional new custom path
    custom_path: Option<String>,
    /// Optional new expiration time in seconds
    expiration: Option<i64>,
}

/// Creates a new shortened URL
/// 
/// This endpoint:
/// 1. Verifies the user's authentication
/// 2. Creates a new shortened URL
/// 3. Returns the created URL data
/// 
/// # Arguments
/// * `body` - The request body containing URL details
/// * `pool` - Database connection pool
/// * `username` - The authenticated user's username
/// 
/// # Returns
/// HTTP response:
/// - 200 OK with the created URL data if successful
/// - 401 Unauthorized if user not found
/// - 500 Internal Server Error if creation fails
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

/// Deletes a shortened URL
/// 
/// This endpoint:
/// 1. Verifies the user's authentication
/// 2. Deletes the specified URL if owned by the user
/// 
/// # Arguments
/// * `id` - The ID of the URL to delete
/// * `pool` - Database connection pool
/// * `username` - The authenticated user's username
/// 
/// # Returns
/// HTTP response:
/// - 204 No Content if successful
/// - 401 Unauthorized if user not found
/// - 500 Internal Server Error if deletion fails
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

/// Lists all shortened URLs for the authenticated user
/// 
/// This endpoint:
/// 1. Verifies the user's authentication
/// 2. Retrieves all URLs owned by the user
/// 
/// # Arguments
/// * `pool` - Database connection pool
/// * `username` - The authenticated user's username
/// 
/// # Returns
/// HTTP response:
/// - 200 OK with the list of URLs if successful
/// - 401 Unauthorized if user not found
/// - 500 Internal Server Error if retrieval fails
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

/// Updates an existing shortened URL
/// 
/// This endpoint:
/// 1. Verifies the user's authentication
/// 2. Updates the specified URL if owned by the user
/// 
/// # Arguments
/// * `pool` - Database connection pool
/// * `username` - The authenticated user's username
/// * `url_data` - The new URL data
/// 
/// # Returns
/// HTTP response:
/// - 200 OK with the updated URL data if successful
/// - 401 Unauthorized if user not found
/// - 500 Internal Server Error if update fails
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
