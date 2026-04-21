use axum::{extract::State, Json,response::IntoResponse, http::StatusCode};

use crate::handlers::AppState;
use crate::models::escrow::Escrow;
use crate::services::{escrow_service, soroban_cli};

#[derive(serde::Deserialize)]
pub struct CreateEscrowRequest {
  pub job_id: String,
  pub amount: f64,
}

pub async fn create(
  State(state): State<AppState>,
  Json(payload): Json<CreateEscrowRequest>,
) -> impl IntoResponse {
  let escrow = Escrow::new(payload.job_id.clone(), payload.amount);

  let invoke_result = soroban_cli::invoke(
    &state.config.contract_id,
    &state.config.source,
    &state.config.network,
    "create_escrow",
    vec![
      ("escrow_id", escrow.id.clone()),
      ("job_id", payload.job_id),
      ("payload", (payload.amount as i128).to_string()),
    ],
  );

  match invoke_result {
    Ok(output) => {
      println!("Soroban Invoke Success: {}", output);

      let col = state.db.collection::<Escrow>("escrows");

      if let Err(e) = escrow_service::save(col, escrow.clone()).await {
        eprintln!("Database Error: {}", e);

        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save into database").into_response();
      }

      (StatusCode::CREATED, Json(escrow)).into_response()
    }

    Err(err) => {
      eprintln!("Soroban Invoke Error: {}", err);

      (StatusCode::BAD_REQUEST, format!("Contract execution failed: {}", err)).into_response()
    }
  }
}
