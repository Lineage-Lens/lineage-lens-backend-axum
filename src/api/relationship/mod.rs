mod post;

use crate::state::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/relationship", post(post::post))
        .with_state(state)
}