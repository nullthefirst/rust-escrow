use mongodb::{Collection, results::{InsertOneResult, UpdateResult}};
use crate::models::escrow::Escrow;

pub async fn save(col: Collection<Escrow>, escrow: Escrow
) -> mongodb::error::Result<InsertOneResult> {
  col.insert_one(escrow, None).await
}

pub async fn update_status(col: Collection<Escrow>, id: &str, status: &str
) -> mongodb::error::Result<UpdateResult> {
  col.update_one(
    mongodb::bson::doc! {"id": id},
    mongodb::bson::doc! {"$set": { "status": status} },
    None,
  ).await
}
