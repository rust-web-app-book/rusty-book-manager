use crate::handler::auth::{login, logout};
use axum::{routing::post, Router};
use registry::AppRegistry;

pub fn routes() -> Router<AppRegistry> {
    let auth_router = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout));
    Router::new().nest("/auth", auth_router)
}
