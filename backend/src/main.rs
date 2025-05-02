mod constants;
mod routes;
mod structs;
mod utils;
use actix_files::{Files, NamedFile};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use constants::POSTGRESQL_URL;
use constants::{FRONTEND_DIST, HOST, PORT};
use dotenv::dotenv;
use routes::health::health;
use routes::register::register;
use sqlx::postgres::PgPoolOptions;
use utils::is_production;

#[get("/")]
async fn development() -> impl Responder {
    HttpResponse::Ok().body("This is running the backend only. The frontend is a separate application. To serve frontend files, you must build the frontend and move the directory containing the built files into the backend.")
}

async fn index() -> NamedFile {
    NamedFile::open((*FRONTEND_DIST).join("index.html")).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // database stuff
    // connecting to database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(*POSTGRESQL_URL)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    // queries to the database
    sqlx::query(
        r#"
        CREATE EXTENSION IF NOT EXISTS "pgcrypto";
    "#,
    )
    .execute(&pool)
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let pool_data = web::Data::new(pool);

    // server stuff
    let server = HttpServer::new(move || {
        let mut app = App::new()
            .service(health)
            .service(register)
            .app_data(pool_data.clone());

        if is_production() {
            // Serve the static HTML files if we are in production
            app = app
                .service(
                    Files::new("/", (*FRONTEND_DIST).to_str().unwrap()).index_file("index.html"),
                )
                .default_service(web::route().to(index));
        } else {
            // Otherwise, warn the user that this route is backend only
            app = app.service(development)
        }

        app
    })
    .bind((*HOST, *PORT))?;

    if !is_production() {
        println!("Did not detect a production environment. Static files will not be served!");
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
