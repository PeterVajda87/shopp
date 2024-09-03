use crate::db::DB_POOL;
use async_trait::async_trait;
use serde::Deserialize;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use super::traits::{FromRequest, Storable};

#[derive(Deserialize, Debug)]
pub struct Language {
    #[serde(default)]
    language_id: Option<Uuid>,
    language_code: String,
    language_name: String,
}

#[async_trait]
impl Storable for Language {
    async fn insert(self) -> Result<PgQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO language
            (language_code, language_name)
            VALUES ($1, $2)"
        );

        sqlx::query(&query)
            .bind(self.language_code)
            .bind(self.language_name)
            .execute(&*DB_POOL)
            .await
    }
}

#[async_trait]
impl FromRequest<Language> for Language {
    async fn create_from_request(
        json_request: ntex::web::types::Json<Language>,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let language = json_request.into_inner();

        Ok(Language {
            language_id: language.language_id.or(Some(Uuid::new_v4())),
            language_code: language.language_code,
            language_name: language.language_name,
        })
    }
}
