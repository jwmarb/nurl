use actix_web::{get, HttpResponse, Responder};

use crate::structs::APIResponse;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(APIResponse::data("alive"))
}
