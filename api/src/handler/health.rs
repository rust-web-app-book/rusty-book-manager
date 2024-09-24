use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

#[cfg_attr(
    debug_assertions,
    utoipa::path(get, path="/api/v1/health",
        responses(
            (status = 200, description = "サーバーが正常に起動している場合。")
        )
)
)]

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(get, path="/api/v1/health/db",
        responses(
            (status = 200, description = "データベースに接続できた場合。"),
            (status = 500, description = "データベースに接続できなかった場合。データベース側に問題があるか、サーバーの設定の問題で接続できていない可能性があります。")
        )
    )
)]
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
