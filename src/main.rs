use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use redis::Client;
use std::sync::Mutex;

mod routes;
mod models;
mod utils;  // Add a utils module for Redis and JWT management

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Redis client
    let redis_client = Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    let redis_data = web::Data::new(utils::redis::RedisUtil { client: redis_client });

    // Shared in-memory state for users
    let data = web::Data::new(routes::AppState {
        users: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()  // Allow all origins for development
                    .allow_any_method()  // Allow all HTTP methods
                    .allow_any_header()  // Allow all headers
                    .max_age(3600),      // Cache CORS preflight response for 1 hour
            )
            .app_data(data.clone())       // In-memory user data
            .app_data(redis_data.clone()) // Redis session data
            .configure(routes::init_routes)  // Initialize routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}