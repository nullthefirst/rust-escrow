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
      mongodb_uri: env::var("MONGODB_URI")
        .expect("MONGODB_URI must be set in .env"),
      db_name: env::var("DB_NAME")
        .expect("DB_NAME must be set in .env"),
      contract_id: env::var("CONTRACT_ID")
        .expect("CONTRACT_ID must be set in .env"),
      source: env::var("STELLAR_SOURCE")
        .expect("STELLAR_SOURCE must be set in .env"),
      network: env::var("STELLAR_NETWORK")
        .expect("STELLAR_NETWORK must be set in .env"),
    }
  }
}
