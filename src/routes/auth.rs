use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use crate::models::user::User;
use crate::utils::jwt;
use crate::utils::redis::RedisUtil;  // Import Redis utility

use super::AppState;

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Register user
pub async fn register(
    user: web::Json<User>,
    data: web::Data<AppState>,
) -> impl Responder {
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    let mut users = data.users.lock().unwrap();
    
    if users.iter().any(|u| u.username == user.username) {
        return HttpResponse::Conflict().json("Username already exists");
    }

    users.push(User {
        username: user.username.clone(),
        password: hashed_password,
    });

    HttpResponse::Ok().json("User registered successfully")
}

// Login user
pub async fn login(
    user: web::Json<User>,
    data: web::Data<AppState>,
    redis: web::Data<RedisUtil>,  // Redis for session storage
) -> impl Responder {
    let users = data.users.lock().unwrap();

    if let Some(stored_user) = users.iter().find(|u| u.username == user.username) {
        if verify(&user.password, &stored_user.password).unwrap() {
            let token = jwt::generate_token(&stored_user.username, "secret");

            // Store the token in Redis with expiration
            redis.store_session(&stored_user.username, &token, 3600).await.unwrap();

            return HttpResponse::Ok().json(token);
        }
    }
    HttpResponse::Unauthorized().json("Invalid username or password")
}

// Logout user (revoke token)
pub async fn logout(
    redis: web::Data<RedisUtil>,
    token: web::Json<String>,
) -> impl Responder {
    redis.revoke_token(&token.into_inner(), 3600).await.unwrap();
    HttpResponse::Ok().json("Token revoked")
}