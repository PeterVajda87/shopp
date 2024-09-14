use super::language::Language;
use super::product::Product;
use super::traits::{FromRequest, Storable};
use crate::db::DB_POOL;
use async_trait::async_trait;
use futures::Future;
use ntex::web::types::Json;
use serde::Deserialize;
use sqlx::{postgres::PgQueryResult, FromRow, Row};
use std::pin::Pin;
use uuid::Uuid;

pub enum CategoryFields {
    Id,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    category_id: Uuid,
    parent_category: Option<Box<Category>>,
    products: Vec<Product>,
    translation: Option<CategoryTranslation>,
}

#[derive(FromRow, Deserialize, Debug)]
pub struct CategoryTranslation {
    category_id: Option<Uuid>,
    language: Language,
    category_name: String,
    category_description: String,
}

struct Query {
    select: Vec<String>,
    left_join: Vec<String>,
    from: String,
    where_clause: Vec<String>,
}

impl Query {
    fn build(self) -> String {
        let mut query = String::new();

        // SELECT clause
        let select_clause = if self.select.is_empty() {
            "*".to_string() // Default to selecting all if nothing is specified
        } else {
            self.select.join(", ")
        };
        query.push_str(&format!("SELECT {} ", select_clause));

        // FROM clause
        query.push_str(&format!("FROM {} ", self.from));

        // LEFT JOIN clause (if any)
        for join in self.left_join {
            query.push_str(&format!("LEFT JOIN {} ", join));
        }

        // WHERE clause (if any)
        if !self.where_clause.is_empty() {
            let where_clause = self.where_clause.join(" AND ");
            query.push_str(&format!("WHERE {} ", where_clause));
        }

        query.trim().to_string() // Remove any trailing spaces
    }
}

struct CategoryJson {}

pub struct CategoryBuilder {
    query: Option<Query>,
    json_payload: Option<Json<CategoryJson>>,
}

impl CategoryBuilder {
    fn from_database_query() -> Self {
        let query = Query {
            from: "category c".to_string(),
            select: vec![
                "c.id as category_id".to_string(),
                "c.parent_category_id as parent_category_id".to_string(),
            ],
            left_join: Vec::new(),
            where_clause: Vec::new(),
        };

        Self {
            query: Some(query),
            json_payload: None,
        }
    }

    fn from_json_paloyad(payload: Json<CategoryJson>) -> Self {
        Self {
            query: None,
            json_payload: Some(payload),
        }
    }

    fn get_by_id(mut self: Self, id: Uuid) -> Self {
        self.query
            .as_mut()
            .unwrap()
            .where_clause
            .push(format!("c.id = {id}"));

        self
    }

    fn with_translation(mut self: Self) -> Self {
        self.query
            .as_mut()
            .unwrap()
            .select
            .push(format!("ct.name, ct.translation"));

        self.query.as_mut().unwrap().left_join.push(format!(
            "category_translation ct ON ct.category_id = c.category_id"
        ));

        self
    }

    async fn build_one(self) -> Result<Category, sqlx::Error> {
        let row = sqlx::query(&self.query.unwrap().build())
            .fetch_one(&*DB_POOL)
            .await?;
        let parent_category_id: Uuid = row.get("parent_category_id");

        if !row.is_empty() {
            let parent_category_future: Pin<
                Box<dyn Future<Output = Result<Option<Category>, sqlx::Error>>>,
            > = if let Some::<Uuid>(_parent_category_id) =
                row.try_get("parent_category_id").unwrap()
            {
                Box::pin(async move {
                    let parent_category = CategoryBuilder::from_database_query()
                        .get_by_id(parent_category_id)
                        .with_translation()
                        .build_one()
                        .await?;
                    Ok(Some(parent_category))
                })
            } else {
                Box::pin(async { Ok(None) })
            };

            let parent_category = parent_category_future.await.unwrap();

            let category = Category {
                category_id: row.get("category_id"),
                parent_category: parent_category.map(|c| Box::new(c)),
                products: Vec::new(),
                translation: None,
            };

            Ok(category)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}

#[async_trait]
impl FromRequest<Category> for Category {
    async fn create_from_request(
        json_request: ntex::web::types::Json<Category>,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let category = json_request.into_inner();

        Ok(Category {
            category_id: category.category_id.or(Some(Uuid::new_v4())),
            parent_category_id: category.parent_category_id.or(None),
            name: category.name.or(None),
            description: category.description.or(None),
            products: None,
            language_code: category.language_code.or(None),
        })
    }
}

#[async_trait]
impl Storable for Category {
    async fn insert(self) -> Result<PgQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO category
            (parent_category_id)
            VALUES ($1)"
        );

        sqlx::query(&query)
            .bind(self.parent_category_id)
            .execute(&*DB_POOL)
            .await
    }
}
