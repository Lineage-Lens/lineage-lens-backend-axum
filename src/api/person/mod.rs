pub mod get;
pub mod post;

use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/person", get(get::get).post(post::post))
        .with_state(state)
}