use redis::AsyncCommands;
use std::convert::TryInto;

pub struct RedisUtil {
    pub client: redis::Client,
}

impl RedisUtil {
    pub async fn store_session(&self, user: &str, token: &str, ttl: usize) -> redis::RedisResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.set_ex(format!("session:{}", user), token, ttl.try_into().unwrap()).await
    }


    pub async fn is_token_revoked(&self, token: &str) -> redis::RedisResult<bool> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let revoked: Option<bool> = conn.get(format!("revoked:{}", token)).await?;
        Ok(revoked.unwrap_or(false))
    }


    pub async fn revoke_token(&self, token: &str, ttl: usize) -> redis::RedisResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
    conn.set_ex(format!("revoked:{}", token), true, ttl.try_into().unwrap()).await
    }
}