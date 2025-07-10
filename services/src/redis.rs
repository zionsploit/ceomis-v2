use redis::{aio::MultiplexedConnection, RedisError};
use tracing::error;

pub struct Redis {
    key: String,
    connection: MultiplexedConnection
}

impl Redis {
    pub fn new(key: String, connection: MultiplexedConnection) -> Self {
        Self { 
            key: key,
            connection: connection
         }
    }

    pub async fn stored_value(&mut self, value: &str) -> Result<bool, RedisError> {

        let stored_value = redis::cmd("SETEX")
            .arg(self.key.as_str()) 
            .arg(3600)
            .arg(value)
            .exec_async(&mut self.connection)
            .await;

        match stored_value {
            Ok(_) => Ok(true),
            Err(err) => {
                error!("Redis SETEX failed for key {}: {}", self.key, err.to_string());
                Err(err)
            }
        }
    }

    pub async fn get_value(&mut self) -> Option<String> {
        let get_stored_value: Result<Option<String>, RedisError> = redis::cmd("GET")
            .arg(self.key.as_str())
            .query_async(&mut self.connection)
            .await;

        match get_stored_value {
            Ok(value) => value,
            Err(_) => None
        }
    }

    pub async fn remove_value(&mut self) -> Result<bool, RedisError> {
        let remove_stored_value = redis::cmd("DEL")
            .arg(self.key.as_str())
            .exec_async(&mut self.connection).await;

        match remove_stored_value {
            Ok(_) => Ok(true),
            Err(err) => {
                error!("Redis DEL failed for key {}", self.key);
                Err(err)
            }
        }
    }

    pub async fn remove_multi_value(&mut self, keys: Vec<&str>) -> Result<bool, RedisError> {
        let mut all_keys = Vec::with_capacity(keys.len() + 1);

        all_keys.push(self.key.as_str());
        all_keys.extend(keys);

        let remove_multi = redis::cmd("DEL").arg(&all_keys).exec_async(&mut self.connection).await;

        match remove_multi {
            Ok(_) => Ok(true),
            Err(err) => Err(err)
        } 
    }

    pub async fn remove_value_if_exists(&mut self) -> Result<(), RedisError> {
        if self.get_value().await.is_some() {
            self.remove_value().await?;
        }

        Ok(())
    }
}

