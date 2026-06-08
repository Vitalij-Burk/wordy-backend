use std::sync::Arc;

use tokio::sync::Mutex;

use crate::domain::error::error_handling::log_err;

#[derive(Debug, Clone)]
pub struct RedisIO<Storage> {
    redis_storage: Arc<Mutex<Storage>>,
}

impl<Storage> RedisIO<Storage>
where
    Storage: redis::AsyncCommands + Send + Sync,
{
    pub fn new(storage: Arc<Mutex<Storage>>) -> Self {
        tracing::info!("Redis initialized");
        Self {
            redis_storage: storage,
        }
    }

    pub async fn set(&self, key: &str, data: &str) -> Result<(), redis::RedisError> {
        let mut storage = self.redis_storage.lock().await;

        storage
            .set::<&str, String, ()>(&key, data.to_string())
            .await
            .map_err(log_err)?;

        Ok(())
    }

    pub async fn setex(&self, key: &str, data: &str, exp: u64) -> Result<(), redis::RedisError> {
        let mut storage = self.redis_storage.lock().await;

        storage
            .set_ex::<&str, String, ()>(&key, data.to_string(), exp)
            .await
            .map_err(log_err)?;

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<String, redis::RedisError> {
        let mut storage = self.redis_storage.lock().await;

        let data = storage.get::<&str, String>(&key).await.map_err(log_err)?;

        Ok(data)
    }

    pub async fn delete(&mut self, key: &str) -> Result<(), redis::RedisError> {
        let mut storage = self.redis_storage.lock().await;

        storage.del::<&str, ()>(&key).await.map_err(log_err)?;

        Ok(())
    }
}
