use super::enums::EntityType;
use crate::db::DB_POOL;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct Slug {
    pub target_id: Uuid,
    pub target_type: EntityType,
    pub text: String,
    pub language_code: String,
}

impl Slug {
    pub async fn get_by_text(text: &str) -> Result<Slug, sqlx::Error> {
        let language_code = "en";
        sqlx::query_as("SELECT target_id, target_type AS \"target_type!: EntityType\", text, language_code FROM slug WHERE text = $1 AND language_code = $2")
            .bind(text)
            .bind(language_code)
            .fetch_one(&*DB_POOL)
            .await
    }
}
