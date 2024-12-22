use super::handlers;
use axum::routing::{get, post};
use axum::Router;

pub fn moon_routes() -> Router<crate::AppState> {
    Router::new()
        .route("/_moon/publish", post(handlers::publish))
        .route("/_moon/publish/:id", post(handlers::republish))
        .route("/_moon/unpublish/:id", post(handlers::unpublish))
        .route("/_moon/detail/:id", get(handlers::detail))
}
