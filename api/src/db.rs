use mongodb::{Client, Database};
use std::sync::Arc;

pub type DB = Arc<Database>;

pub async fn init_db(uri: &str, db_name: &str) -> DB {
  let client = Client::with_uri_str(uri).await.unwrap();
  Arc::new(client.database(db_name))
}
