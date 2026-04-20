use mongodb::{Client, Database};
use std::Async::Arc;

pub type DB = Arc<Database>;

pub async fn init_db(uri: &str, db_name: &str) -> DB {
  let client = Client::with_uri_string(uri).unwrap();
  Arc::new(client.database(db_name));
}
