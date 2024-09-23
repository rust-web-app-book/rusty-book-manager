use crate::{
    database::{
        model::auth::{from, AuthorizationKey, AuthorizedUserId, UserItem},
        ConnectionPool,
    },
    redis::RedisClient,
};
use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        auth::{event::CreateToken, AccessToken},
        id::UserId,
    },
    repository::auth::AuthRepository,
};
use shared::error::{AppError, AppResult};
use std::sync::Arc;

#[derive(new)]
pub struct AuthRepositoryImpl {
    db: ConnectionPool,
    kv: Arc<RedisClient>,
    ttl: u64,
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn fetch_user_id_from_token(
        &self,
        access_token: &AccessToken,
    ) -> AppResult<Option<UserId>> {
        let key: AuthorizationKey = access_token.into();
        self.kv
            .get(&key)
            .await
            .map(|x| x.map(AuthorizedUserId::into_inner))
    }

    async fn verify_user(&self, email: &str, password: &str) -> AppResult<UserId> {
        let user_item = sqlx::query_as!(
            UserItem,
            r#"
                SELECT user_id, password_hash FROM users
                WHERE email = $1;
            "#,
            email
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        let valid = bcrypt::verify(password, &user_item.password_hash)?;
        if !valid {
            return Err(AppError::UnauthenticatedError);
        }
        Ok(user_item.user_id)
    }

    async fn create_token(&self, event: CreateToken) -> AppResult<AccessToken> {
        let (key, value) = from(event);
        self.kv.set_ex(&key, &value, self.ttl).await?;
        Ok(key.into())
    }

    async fn delete_token(&self, access_token: AccessToken) -> AppResult<()> {
        let key: AuthorizationKey = access_token.into();
        self.kv.delete(&key).await
    }
}
