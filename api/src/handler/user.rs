use crate::{
    extractor::AuthorizedUser,
    model::user::{
        CreateUserRequest, UpdateUserPasswordRequest, UpdateUserPasswordRequestWithUserId,
        UpdateUserRoleRequest, UpdateUserRoleRequestWithUserId, UserResponse, UsersResponse,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::model::{id::UserId, user::event::DeleteUser};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

/// ユーザーを追加する（Admin only）
#[tracing::instrument(
    skip(user, registry, req),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn register_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    //AuthorizedUser の権限が Admin のときのみ実行可能とする
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }
    req.validate(&())?;

    let registered_user = registry.user_repository().create(req.into()).await?;

    Ok(Json(registered_user.into()))
}

/// ユーザーの一覧を取得する
#[cfg_attr(
    debug_assertions,
    utoipa::path(get, path="/api/v1/users",
        responses(
            (status = 200, description = "ユーザーの一覧を取得できた場合。"),
            (status = 500, description = "サーバーサイドエラーが発生した場合。")
        )
    )
)]
pub async fn list_users(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<UsersResponse>> {
    let items = registry
        .user_repository()
        .find_all()
        .await?
        .into_iter()
        .map(UserResponse::from)
        .collect();

    Ok(Json(UsersResponse { items }))
}

/// ユーザーを削除する（Admin only）
#[tracing::instrument(
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn delete_user(
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    //AuthorizedUser の権限が Admin のときのみ実行可能とする
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    registry
        .user_repository()
        .delete(DeleteUser { user_id })
        .await?;

    Ok(StatusCode::OK)
}

/// ユーザーのロールを変更する（Admin only）
pub async fn change_role(
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> AppResult<StatusCode> {
    //AuthorizedUser の権限が Admin のときのみ実行可能とする
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    registry
        .user_repository()
        .update_role(UpdateUserRoleRequestWithUserId::new(user_id, req).into())
        .await?;

    Ok(StatusCode::OK)
}

/// ユーザーが自分自身のユーザー情報を取得する
#[cfg_attr(
    debug_assertions,
    utoipa::path(get, path="/api/v1/users/me",
        responses(
            (status = 200, description = "現在ログイン中のユーザー情報の取得に成功した場合。")
        )
    )
)]
#[tracing::instrument(
    skip(user),
    fields(
        user_id = %user.user.id.to_string(),
        user_name = %user.user.name
    )
)]
pub async fn get_current_user(user: AuthorizedUser) -> Json<UserResponse> {
    Json(UserResponse::from(user.user))
}

/// ユーザーが自分自身のパスワードを変更する
#[cfg_attr(
    debug_assertions,
    utoipa::path(get, path="/api/v1/users/me/password",
        responses(
            (status = 200, description = "パスワードの変更に成功した場合。"),
            (status = 400, description = "リクエストの形式に誤りがある場合。"),
            (status = 500, description = "サーバーサイドエラーが発生した場合。")
        )
    )
)]
#[tracing::instrument(
    skip(user, registry, req),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn change_password(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserPasswordRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    registry
        .user_repository()
        .update_password(UpdateUserPasswordRequestWithUserId::new(user.id(), req).into())
        .await?;

    Ok(StatusCode::OK)
}

use crate::model::checkout::CheckoutsResponse;
/// 追加する関数
/// ユーザーが自身の借りている書籍の一覧を取得する
#[cfg_attr(
    debug_assertions,
    utoipa::path(get, path="/api/v1/users/me/checkouts",
        responses(
            (status = 200, description = "貸し出し中の書籍を取得できた場合。"),
            (status = 500, description = "サーバーサイドエラーが発生した場合。")
        )
    )
)]
#[tracing::instrument(
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn get_checkouts(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .checkout_repository()
        .find_unreturned_by_user_id(user.id())
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}
