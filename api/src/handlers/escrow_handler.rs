use axum::{Extract::State, Json};
use std::sync::Arc;
use mongodb::Database;

use crate::models::escrow::Escrow;
use crate::services::{escrow_service, soroban_cli};
use crate::config::Config;

#[derive(serde::Deserialize)]
pub struct CreateEscrowRequest {
  pub job_id: String,
  pub amount: f64,
}

pub async fn create(
  State((db, config)): State<(Arc<Database>, Config)>,
  Json(payload): Json<CreateEscrowRequest>,
) -> Json<Escrow> {
  let escrow = Escrow::new(payload.job_id.clone(), payload.amount);

  let _ = soroban_cli::invoke(
    &config.contract_id,
    &config.source,
    &config.network,
    "create_escrow",
    vec![
      ("escrow_id", escrow.id.clone()),
      ("job_id", payload.job_id()),
      ("payload", (payload.amount as i128).to_string()),
    ],
  );

  let col = db.Collection::<Escrows>("escrows");
  escrow_service::save(col, escrow::clone()).await;

  Json(escrow)
}
