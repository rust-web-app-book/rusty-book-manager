use crate::model::{
    id::UserId,
    user::{
        event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole},
        User,
    },
};
use async_trait::async_trait;
use shared::error::AppResult;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_current_user(&self, current_user_id: UserId) -> AppResult<Option<User>>;
    async fn find_all(&self) -> AppResult<Vec<User>>;
    async fn create(&self, event: CreateUser) -> AppResult<User>;
    async fn update_password(&self, event: UpdateUserPassword) -> AppResult<()>;
    async fn update_role(&self, event: UpdateUserRole) -> AppResult<()>;
    async fn delete(&self, event: DeleteUser) -> AppResult<()>;
}
