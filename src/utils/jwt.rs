//helper functions for generating and verifying JWT tokens to keep auth.rs clean.use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::utils::redis::RedisUtil;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, };  // Add this import
use std::convert::TryInto;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (username)
    pub exp: usize,   // Expiration timestamp
}

pub fn generate_token(username: &str, secret: &str) -> String {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub async fn validate_token(token: &str, secret: &str, redis_util: &RedisUtil) -> Result<Claims, String> {
    if redis_util.is_token_revoked(token).await.unwrap_or(false) {
        return Err("Token is revoked".to_string());
    }

    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| "Invalid token".to_string())
}