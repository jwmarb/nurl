mod constants;
mod middleware;
mod routes;
mod service;
mod structs;
mod utils;
use actix_files as fs;
use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use constants::{FRONTEND_DIST, HOST, PORT};
use dotenv::dotenv;
use middleware::ExtractUsernameJWT;
use routes::auth::is_authenticated;
use routes::redirect::redirect_to_original_url;
use routes::register::register;
use routes::shorten::{
    delete_shortened_url, get_shortened_urls, shorten_url, update_shortened_url,
};
use routes::{auth::login, health::health};
use serde_json::Value;
use std::fs as std_fs;
use utils::{init_db, is_production};

#[get("/")]
async fn development() -> impl Responder {
    HttpResponse::Ok().body("This is running the backend only. The frontend is a separate application. To serve frontend files, you must build the frontend and move the directory containing the built files into the backend.")
}

// Serve "/"
#[get("/")]
async fn serve_index() -> impl Responder {
    let path = format!(
        "{}/client/index.html",
        FRONTEND_DIST.to_string_lossy().to_string(),
    );
    fs::NamedFile::open(path)
}

// Serve "/auth"
#[get("/auth")]
async fn serve_auth() -> impl Responder {
    let path = format!(
        "{}/client/auth/index.html",
        FRONTEND_DIST.to_string_lossy().to_string(),
    );
    fs::NamedFile::open(path)
}

// Serve "/auth/register"
#[get("/auth/register")]
async fn serve_auth_register() -> impl Responder {
    let path = format!(
        "{}/client/auth/register/index.html",
        FRONTEND_DIST.to_string_lossy().to_string(),
    );
    fs::NamedFile::open(path)
}

#[get("/assets/{filename:.*}")]
async fn serve_assets(path: actix_web::web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    let path = format!(
        "{}/client/assets/{}",
        FRONTEND_DIST.to_string_lossy().to_string(),
        filename
    );
    fs::NamedFile::open(path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = init_db().await.map(|p| web::Data::new(p))?;

    let server = HttpServer::new(move || {
        let mut app = App::new()
            .service(health)
            .service(register)
            .service(login)
            .service(
                web::scope("/api")
                    .wrap(ExtractUsernameJWT)
                    .service(shorten_url)
                    .service(delete_shortened_url)
                    .service(get_shortened_urls)
                    .service(update_shortened_url),
            )
            .service(is_authenticated)
            .service(redirect_to_original_url)
            .app_data(pool.clone());

        if is_production() {
            // Serve the static HTML files if we are in production
            app = app
                .service(serve_auth)
                .service(serve_auth_register)
                .service(serve_index)
                .service(serve_assets)
        } else {
            // Otherwise, warn the user that this route is backend only
            app = app.service(development);
        }

        app
    })
    .bind((*HOST, *PORT))?;

    if !is_production() {
        println!("Did not detect a production environment. Static files will not be served!");
    } else {
        println!("Detected production environment");
    }

    println!(
        "Server started on {HOST}:{PORT}",
        HOST = *HOST,
        PORT = *PORT
    );

    server.run().await
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_main() {
        assert!(true);
    }
}
