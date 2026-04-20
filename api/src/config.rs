use std::env;

#[derive(Clone)]
pub struct Config {
  pub mongodb_uri: String,
  pub db_name: String,
  pub contract_id: String,
  pub source: String,
  pub network: String,
}

impl Config {
  pub fn from_env() -> Self {
    Self {
      mongodb_uri: env::var("MONGODB_URI").unwrap(),
      db_name: env::var("DB_NAME").unwrap(),
      contract_id: env::var("CONTRACT_ID").unwrap(),
      source: env::var("STELLAR_SOURCE").unwrap(),
      network: env::var("STELLAR_NETWORK").unwrap(),
    }
  }
}
