use model::{RedisKey, RedisValue, RedisValueError};
use redis::AsyncCommands;
use shared::config::RedisConfig;

pub mod model;

pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub fn new(config: &RedisConfig) -> RedisClientResult<Self> {
        let client = redis::Client::open(format!("redis://{}:{}", config.host, config.port))?;
        Ok(Self { client })
    }

    pub async fn set_ex<T: RedisKey>(
        &self,
        key: &T,
        value: &T::Value,
        ttl: u64,
    ) -> RedisClientResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let _: () = conn.set_ex(key.inner(), value.inner(), ttl).await?;
        Ok(())
    }

    pub async fn get<T: RedisKey>(&self, key: &T) -> RedisClientResult<Option<T::Value>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value: Option<String> = conn.get(key.inner()).await?;
        Ok(value.map(T::Value::try_from).transpose()?)
    }

    pub async fn delete<T: RedisKey>(&self, key: &T) -> RedisClientResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let _: () = conn.del(key.inner()).await?;
        Ok(())
    }

    // ヘルスチェック用の接続確認関数
    pub async fn try_connect(&self) -> RedisClientResult<()> {
        let _ = self.client.get_multiplexed_async_connection().await?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RedisClientError {
    #[error("redis value error: {0}")]
    RedisValueError(#[from] RedisValueError),

    #[error("redis error: {0}")]
    RedisError(#[from] redis::RedisError),
}

pub type RedisClientResult<T> = Result<T, RedisClientError>;
