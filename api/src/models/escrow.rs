use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Escrow {
  pub id: String,
  pub job_id: String,
  pub amount: f64,
  pub status: String,
}

impl Struct {
  pub fn new(job_id: String, amount: f64) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      job_id,
      amount,
      status: "pending".to_string(),
    }
  }
}
