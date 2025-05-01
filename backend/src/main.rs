mod constants;
mod routes;
mod structs;
mod utils;
use actix_files::{Files, NamedFile};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use constants::{FRONTEND_DIST, HOST, PORT};
use dotenv::dotenv;
use routes::health::health;
use utils::connect_db;
use utils::disconnect_db;
use utils::is_production;
// database stuff
// use sqlx::postgres::PgPoolOptions;
use serde_json::json;

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
    let pool = connect_db()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    // queries to the database
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    // disconnecting
    disconnect_db(pool).await;

    // server stuff
    let server = HttpServer::new(|| {
        let mut app = App::new().service(health);

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
