pub mod model;

use self::model::{RedisKey, RedisValue};
use redis::{AsyncCommands, Client};
use shared::{config::RedisConfig, error::AppResult};

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(config: &RedisConfig) -> AppResult<Self> {
        let client = Client::open(format!("redis://{}:{}", config.host, config.port))?;
        Ok(Self { client })
    }

    pub async fn set_ex<T: RedisKey>(&self, key: &T, value: &T::Value, ttl: u64) -> AppResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.set_ex(key.inner(), value.inner(), ttl).await?;
        Ok(())
    }

    pub async fn get<T: RedisKey>(&self, key: &T) -> AppResult<Option<T::Value>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result: Option<String> = conn.get(key.inner()).await?;
        result.map(T::Value::try_from).transpose()
    }

    pub async fn delete<T: RedisKey>(&self, key: &T) -> AppResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.del(key.inner()).await?;
        Ok(())
    }

    pub async fn try_connect(&self) -> AppResult<()> {
        let _ = self.client.get_multiplexed_async_connection().await?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use shared::error::AppError;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TestContent {
        pub name: String,
    }

    pub struct TestContentKey(String);

    impl RedisKey for TestContentKey {
        type Value = TestContent;
        fn inner(&self) -> String {
            self.0.to_string()
        }
    }

    impl TryFrom<String> for TestContent {
        type Error = AppError;

        fn try_from(s: String) -> Result<Self, Self::Error> {
            Ok(Self { name: s })
        }
    }

    impl RedisValue for TestContent {
        fn inner(&self) -> String {
            self.name.to_string()
        }
    }

    #[sqlx::test]
    async fn test_con() -> anyhow::Result<()> {
        let config = RedisConfig {
            host: std::env::var("REDIS_HOST").unwrap(),
            port: std::env::var("REDIS_PORT").unwrap().parse().unwrap(),
        };
        let client = RedisClient::new(&config)?;

        // 存在しないトークンを取得しようとする
        let res_nonexist = client.get(&TestContentKey("redis:key".to_string())).await?;
        assert!(res_nonexist.is_none());

        // トークンを設定
        client
            .set_ex(
                &TestContentKey("redis:key".to_string()),
                &TestContent {
                    name: "bbb".to_string(),
                },
                1000,
            )
            .await?;

        // トークンを取得できる
        let res = client.get(&TestContentKey("redis:key".to_string())).await?;
        assert_eq!(
            res,
            Some(TestContent {
                name: "bbb".to_string()
            })
        );

        // トークンを削除
        client
            .delete(&TestContentKey("redis:key".to_string()))
            .await?;

        // トークンを取得できない
        let res_nonexist = client.get(&TestContentKey("redis:key".to_string())).await?;
        assert!(res_nonexist.is_none());

        Ok(())
    }
}
