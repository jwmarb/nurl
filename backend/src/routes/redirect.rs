use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::structs::ShortenedUrl;

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
