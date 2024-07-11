use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub category_id: Uuid,
    pub created_at: DateTime<Utc>,
}
