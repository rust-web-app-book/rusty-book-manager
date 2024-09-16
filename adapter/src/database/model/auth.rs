use shared::error::{AppError, AppResult};
use std::str::FromStr;

use kernel::model::{
    auth::{event::CreateToken, AccessToken},
    id::UserId,
};

use crate::redis::model::{RedisKey, RedisValue};

pub struct UserItem {
    pub user_id: UserId,
    pub password_hash: String,
}

pub struct AuthorizationKey(String);
pub struct AuthorizedUserId(UserId);

pub fn from(event: CreateToken) -> (AuthorizationKey, AuthorizedUserId) {
    (
        AuthorizationKey(event.access_token),
        AuthorizedUserId(event.user_id),
    )
}

impl From<AuthorizationKey> for AccessToken {
    fn from(key: AuthorizationKey) -> Self {
        Self(key.0)
    }
}

impl From<AccessToken> for AuthorizationKey {
    fn from(token: AccessToken) -> Self {
        Self(token.0)
    }
}

impl From<&AccessToken> for AuthorizationKey {
    fn from(token: &AccessToken) -> Self {
        Self(token.0.to_string())
    }
}

impl RedisKey for AuthorizationKey {
    type Value = AuthorizedUserId;

    fn inner(&self) -> String {
        self.0.clone()
    }
}

impl RedisValue for AuthorizedUserId {
    fn inner(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<String> for AuthorizedUserId {
    type Error = AppError;

    fn try_from(s: String) -> AppResult<Self> {
        Ok(Self(UserId::from_str(&s).map_err(|e| {
            AppError::ConversionEntityError(e.to_string())
        })?))
    }
}

impl AuthorizedUserId {
    pub fn into_inner(self) -> UserId {
        self.0
    }
}
