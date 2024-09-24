use crate::{
    extractor::AuthorizedUser,
    model::book::{
        BookListQuery, BookResponse, CreateBookRequest, PaginatedBookResponse, UpdateBookRequest,
        UpdateBookRequestWithIds,
    },
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::model::{book::event::DeleteBook, id::BookId};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

#[cfg_attr(
    debug_assertions,
    utoipa::path(post, path="/api/v1/books",
        request_body = CreateBookRequest,
        responses(
            (status = 201, description = "蔵書の登録に成功した場合。"),
            (status = 400, description = "リクエストのパラメータに不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 422, description = "リクエストした蔵書の登録に失敗した場合。")
        )
    )
)]
#[tracing::instrument(
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string()
    )
)]
pub async fn register_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
    req.validate(&())?;

    registry
        .book_repository()
        .create(req.into(), user.id())
        .await
        .map(|_| StatusCode::CREATED)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path="/api/v1/books",
        responses(
            (status = 200, description = "蔵書一覧の取得に成功した場合。", body = PaginatedBookResponse),
            (status = 400, description = "指定されたクエリの値に不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
        ),
        params(
            ("limit" = i64, Query, description = "一度に取得する蔵書数の上限値の指定"),
            ("offset" = i64, Query, description = "取得対象とする蔵書一覧の開始位置"),
        )
    )
)]
#[tracing::instrument(
    skip(_user, registry),
    fields(
        user_id = %_user.user.id.to_string()
    )
)]
pub async fn show_book_list(
    _user: AuthorizedUser,
    Query(query): Query<BookListQuery>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<PaginatedBookResponse>> {
    query.validate(&())?;

    registry
        .book_repository()
        .find_all(query.into())
        .await
        .map(PaginatedBookResponse::from)
        .map(Json)
}

#[cfg_attr(
    debug_assertions,
    // 2
    utoipa::path(
        get,
        path="/api/v1/books",
        responses(
            (status = 200, description = "蔵書一覧の取得に成功した場合。", body = PaginatedBookResponse),
            (status = 400, description = "指定されたクエリの値に不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
        ),
        params(
            ("limit" = i64, Query, description = "一度に取得する蔵書数の上限値の指定"),
            ("offset" = i64, Query, description = "取得対象とする蔵書一覧の開始位置"),
        )
    )
)]
#[tracing::instrument(
    skip(_user, registry),
    fields(
        user_id = %_user.user.id.to_string()
    )
)]
pub async fn show_book(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BookResponse>> {
    tracing::info!("ここにログを追加した");
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(AppError::EntityNotFound(
                "The specific book was not found".to_string(),
            )),
        })
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(put, path="/api/v1/books/{book_id}",
        request_body = UpdateBookRequest,
        responses(
            (status = 200, description = "蔵書の更新に成功した場合。"),
            (status = 400, description = "リクエストのパラメータに不備があった場合。"),
            (status = 404, description = "変更対象の書籍が見つからなかった場合。")
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
pub async fn update_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    let update_book = UpdateBookRequestWithIds::new(book_id, user.id(), req);
    registry
        .book_repository()
        .update(update_book.into())
        .await
        .map(|_| StatusCode::OK)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(delete, path="/api/v1/books/{book_id}",
        responses(
            (status = 204, description = "書籍の削除に成功した場合。"),
            (status = 400, description = "リクエストのパラメータが不正だった場合。"),
            (status = 404, description = "削除対象の書籍が存在しなかった場合。"),
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
pub async fn delete_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let delete_book = DeleteBook {
        book_id,
        requested_user: user.id(),
    };
    registry
        .book_repository()
        .delete(delete_book)
        .await
        .map(|_| StatusCode::OK)
}
