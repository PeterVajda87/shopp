use super::product::Product;
use super::traits::{AddProducts, AddSkus, HasId, HasProducts};
use crate::db::DB_POOL;
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct Category {
    category_id: Uuid,
    parent_categoery_id: Option<Uuid>,
    name: String,
    description: String,
    #[sqlx(skip)]
    products: Vec<Product>,
}

impl HasProducts for Category {}
impl HasId for Category {}

#[async_trait]
impl AddProducts for Category {
    async fn add_products(mut self) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let product_uuids_query_str = format!(
            "SELECT product_id
            FROM product_to_category
            WHERE category_id = $1"
        );

        let product_ids: Vec<Uuid> = sqlx::query_scalar(&product_uuids_query_str)
            .bind(self.category_id)
            .fetch_all(&*DB_POOL)
            .await?;

        self.products = Product::filter_by_id(product_ids).await.unwrap();

        Ok(self)
    }
}

#[async_trait]
impl AddProducts for Vec<Category> {
    async fn add_products(mut self) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        for category in &mut self {
            let product_ids: Vec<Uuid> = sqlx::query_scalar(
                "SELECT product_id FROM product_to_category WHERE category_id = $1",
            )
            .bind(category.category_id)
            .fetch_all(&*DB_POOL)
            .await?;

            category.products = Product::filter_by_id(product_ids)
                .await
                .unwrap()
                .add_skus()
                .await
                .unwrap();
        }

        Ok(self)
    }
}

impl Category {
    pub async fn get_by_id(id: Uuid) -> Result<Category, sqlx::Error> {
        let language_code = "en";
        sqlx::query_as(
            "SELECT
                c.category_id,
                c.parent_category_id,
                ct.category_name,
                ct.category_description
            FROM
                category c
            LEFT JOIN
                category_translation ct
            ON
                c.category_id = ct.category_id
                AND ct.language_code = $1
            WHERE
                c.category_id = $2",
        )
        .bind(language_code)
        .bind(id)
        .fetch_one(&*DB_POOL)
        .await
    }

    pub async fn filter_by_id(ids: Vec<Uuid>) -> Result<Vec<Category>, sqlx::Error> {
        let language_code = "en";
        sqlx::query_as(
            "SELECT
                c.category_id,
                c.parent_category_id,
                ct.category_name,
                ct.category_description
            FROM
                category c
            LEFT JOIN
                category_translation ct
            ON
                c.category_id = ct.category_id
                AND ct.language_code = $1
            WHERE
                c.category_id IN $2",
        )
        .bind(language_code)
        .bind(ids)
        .fetch_all(&*DB_POOL)
        .await
    }
}
