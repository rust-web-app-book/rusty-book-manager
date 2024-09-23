use super::{
    book::build_book_routers, health::build_health_check_routers, user::build_user_router,
};
use axum::Router;
use registry::AppRegistry;

pub fn routes() -> Router<AppRegistry> {
    let router = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routers())
        .merge(build_user_router());
    Router::new().nest("/api/v1", router)
}
