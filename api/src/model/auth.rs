use kernel::model::id::UserId;
use serde::{Deserialize, Serialize};
#[cfg(debug_assertions)]
use utoipa::ToSchema;

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenResponse {
    pub user_id: UserId,
    pub access_token: String,
}
