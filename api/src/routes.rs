use axum::{Router, routing::{get, post}};
use std::sync::Arc;
use mongodb::Database;

use crate::handlers::escrow_handler;
use crate::config::Config;

pub fn create_routes(
  db: Arc<Database>,
  config: Config,
) -> Router {

  Router::new()
    .route("/health", get(|| async { "API Live!" }))
    .route("/escrow", post(escrow_handler::create))
    .with_state((db, config))

}
