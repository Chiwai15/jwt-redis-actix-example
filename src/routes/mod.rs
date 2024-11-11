pub mod auth;  // Expose the auth module
use actix_web::web;
use crate::models::user::User;  // Use fully qualified path to public struct

pub struct AppState {
    pub users: std::sync::Mutex<Vec<User>>,  // Shared state
}

// Function to initialize routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")  // Define base route
            .route("/register", web::post().to(auth::register))
            .route("/login", web::post().to(auth::login))
            .route("/logout", web::post().to(auth::logout))
    );
}