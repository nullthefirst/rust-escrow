use axum::{Router, routing::{get, post}};

use crate::handlers::{escrow_handler, AppState};

pub fn create_routes(state: AppState) -> Router {

  Router::new()
    .route("/health", get(|| async { "API Live!" }))
    // .route("/escrow", post(escrow_handler::create))
    .with_state(state)

}
