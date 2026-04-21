pub mod escrow_handler;

#[derive(Clone)]
pub struct AppState {
  pub db: mongodb::Database,
  pub config: crate::config::Config,
}
