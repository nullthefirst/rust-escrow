use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
  pub id: String,
  pub title: String,
  pub budget: f64,
}

impl Job {
  pub fn new(title: String, budget: f64) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      title,
      budget,
    }
  }
}
