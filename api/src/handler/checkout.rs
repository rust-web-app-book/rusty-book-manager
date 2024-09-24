use crate::{extractor::AuthorizedUser, model::checkout::CheckoutsResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use kernel::model::{
    checkout::event::{CreateCheckout, UpdateReturned},
    id::{BookId, CheckoutId},
};
use registry::AppRegistry;
use shared::error::AppResult;

#[cfg_attr(
    debug_assertions,
    utoipa::path(post, path="/api/v1/books/{book_id}/checkouts",
        responses(
            (status = 201, description = "貸出の登録に成功した場合。"),
            (status = 400, description = "リクエストのパラメータが不正な場合。"),
            (status = 422, description = "リクエストされた処理が実行できない場合。"),
            (status = 500, description = "貸出の登録に失敗した場合。")
        ),
        params(
            ("book_id" = Uuid, Path, description = "蔵書ID")
        )
    )
)]
#[tracing::instrument(
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string()
    )
)]
pub async fn checkout_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let create_checkout_history = CreateCheckout::new(book_id, user.id(), chrono::Utc::now());

    registry
        .checkout_repository()
        .create(create_checkout_history)
        .await
        .map(|_| StatusCode::CREATED)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(put, path="/api/v1/books/{book_id}/checkouts/{checkout_id}/returned",
        responses(
            (status = 200, description = "返却に成功した場合。"),
            (status = 400, description = "リクエストのパラメータが不正な場合。"),
            (status = 422, description = "リクエストされた処理が実行できない場合。"),
            (status = 500, description = "返却の登録に失敗した場合。")
        ),
        params(
            ("book_id" = Uuid, Path, description = "蔵書ID"),
            ("checkout_id" = Uuid, Path, description = "貸出ID")
        )
    )
)]
#[tracing::instrument(
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string()
    )
)]
pub async fn return_book(
    user: AuthorizedUser,
    Path((book_id, checkout_id)): Path<(BookId, CheckoutId)>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let update_returned = UpdateReturned::new(checkout_id, book_id, user.id(), chrono::Utc::now());

    registry
        .checkout_repository()
        .update_returned(update_returned)
        .await
        .map(|_| StatusCode::OK)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(get, path="/api/v1/books/checkouts",
        responses(
            (status = 200, description = "蔵書の貸し出し履歴の一覧取得に成功した場合。", body = CheckoutsResponse),
        )
    )
)]
#[tracing::instrument(
    skip(_user, registry),
    fields(
        user_id = %_user.user.id.to_string()
    )
)]
pub async fn show_checked_out_list(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .checkout_repository()
        .find_unreturned_all()
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(get, path="/api/v1/books/{book_id}/checkout-history",
        responses(
            (status = 200, description = "蔵書の貸し出し履歴の一覧取得に成功した場合。", body = CheckoutsResponse),
        ),
        params(
            ("book_id" = Uuid, Path, description = "蔵書ID")
        )
    )
)]
#[tracing::instrument(
    skip(_user, registry),
    fields(
        user_id = %_user.user.id.to_string()
    )
)]
pub async fn checkout_history(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .checkout_repository()
        .find_history_by_book_id(book_id)
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}
