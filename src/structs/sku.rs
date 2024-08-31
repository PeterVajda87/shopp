use crate::db::DB_POOL;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, sqlx::Type, Serialize, Deserialize)]
pub struct Sku {
    sku_id: Uuid,
    name: String,
    description: String,
}

impl Sku {
    pub async fn get_by_id(id: Uuid) -> Result<Sku, sqlx::Error> {
        let language_code = "en";
        sqlx::query_as("SELECT s.sku_id, st.sku_name, st.sku_description FROM sku s LEFT JOIN sku_translation st ON s.sku_id = st.sku_id AND st.language_code = $1 WHERE sku_id = $2")
            .bind(language_code)
            .bind(id)
            .fetch_one(&*DB_POOL)
            .await
    }

    pub async fn filter_by_id(ids: Vec<Uuid>) -> Result<Vec<Sku>, sqlx::Error> {
        let language_code = "en";
        sqlx::query_as("SELECT s.sku_id, st.sku_name, st.sku_description FROM sku s LEFT JOIN sku_translation st ON s.sku_id = st.sku_id AND st.language_code = $1 WHERE sku_id IN $2")
            .bind(language_code)
            .bind(ids)
            .fetch_all(&*DB_POOL)
            .await
    }
}
