use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware::Logger};
use dotenv::dotenv;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;
use std::path::Path;
use std::sync::Arc;

// Simple test handler
async fn index() -> impl Responder {
    log_message("Accessed / endpoint");
    HttpResponse::Ok().body("Welcome to SKDresses API")
}

// Shared logger (Mutex protected)
struct LogState {
    file: Mutex<File>,
}

// Utility function to log messages
fn log_message(msg: &str) {
    let log_path = Path::new("logs/server.log");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .unwrap();
    let timestamp = chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    writeln!(file, "{} {}", timestamp, msg).unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env

    // Read SECRET_KEY from .env
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    // Ensure log directory exists
    std::fs::create_dir_all("logs").expect("Failed to create logs directory");

    log_message(&format!("Server starting with SECRET_KEY: {}", secret));

    // Start Actix server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Log access logs to stdout (optional)
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
