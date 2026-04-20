use mongodb::Collection;
use crate::models::escrow::Escrow;

pub async fn save(col: Collection<Escrow>, escrow: Escrow) {
  col.insert_one(escrow, None).await.unwrap();
}

pub async fn update_status(col: Collection<Escrow>, id: &str, status: &str) {
  col.update_one(
    mongodb::bson::doc! {"id": id},
    mongodb::bson::doc! {"$set": { "status": status} },
    None,
  ).await.unwrap();
}
