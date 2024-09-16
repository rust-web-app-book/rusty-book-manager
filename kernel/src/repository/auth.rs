use crate::model::{
    auth::{event::CreateToken, AccessToken},
    id::UserId,
};
use async_trait::async_trait;
use shared::error::AppResult;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn fetch_user_id_from_token(
        &self,
        access_token: &AccessToken,
    ) -> AppResult<Option<UserId>>;
    async fn verify_user(&self, email: &str, password: &str) -> AppResult<UserId>;
    async fn create_token(&self, event: CreateToken) -> AppResult<AccessToken>;
    async fn delete_token(&self, access_token: AccessToken) -> AppResult<()>;
}
