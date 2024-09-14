use super::sku::Sku;
use super::traits::{AddSkus, HasId, HasSkus};
use crate::db::DB_POOL;
use async_trait::async_trait;
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Deserialize, Debug)]
pub struct Product {
    product_id: Uuid,
    name: String,
    description: String,
    #[sqlx(skip)]
    skus: Vec<Sku>,
}

impl HasSkus for Product {}
impl HasSkus for Vec<Product> {}
impl HasId for Product {}

#[async_trait]
impl AddSkus for Product {
    async fn add_skus(mut self) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let language_code = "en";
        let sku_uuids_query_str = format!(
            "SELECT sku_id
            FROM sku_to_product
            WHERE product_id = $1"
        );

        let sku_ids: Vec<Uuid> = sqlx::query_scalar(&sku_uuids_query_str)
            .bind(self.product_id)
            .fetch_all(&*DB_POOL)
            .await?;

        let skus_query_str = format!(
            "SELECT s.sku_id, st.sku_name, st.sku_description
            FROM sku s
            LEFT JOIN sku_translation st ON s.sku_id = st.sku_id AND st.language_code = $1
            WHERE s.sku_id IN $2"
        );

        let skus: Vec<Sku> = sqlx::query_as(&skus_query_str)
            .bind(language_code)
            .bind(sku_ids)
            .fetch_all(&*DB_POOL)
            .await?;

        self.skus = skus;

        Ok(self)
    }
}

#[async_trait]
impl AddSkus for Vec<Product> {
    async fn add_skus(mut self) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let language_code = "en";

        for product in &mut self {
            let sku_ids: Vec<Uuid> =
                sqlx::query_scalar("SELECT sku_id FROM sku_to_product WHERE product_id = $1")
                    .bind(product.product_id)
                    .fetch_all(&*DB_POOL)
                    .await?;

            let skus: Vec<Sku> = sqlx::query_as(
                    "SELECT s.sku_id, st.sku_name, st.sku_description FROM sku s LEFT JOIN sku_translation st ON s.sku_id = st.sku_id AND st.language_code = $1 WHERE s.sku_id IN $2"
                )
                .bind(language_code)
                .bind(sku_ids)
                .fetch_all(&*DB_POOL)
                .await?;

            product.skus = skus;
        }

        Ok(self)
    }
}

impl Product {
    pub async fn get_by_id(id: Uuid) -> Result<Product, sqlx::Error> {
        let language_code = "en";
        sqlx::query_as("SELECT p.product_id, pt.product_name, pt.product_description FROM product p LEFT JOIN product_translation pt ON p.sku_id = pt.sku_id AND pt.language_code = $1 WHERE product_id = $2")
            .bind(language_code)
            .bind(id)
            .fetch_one(&*DB_POOL)
            .await
    }

    pub async fn filter_by_id(ids: Vec<Uuid>) -> Result<Vec<Product>, sqlx::Error> {
        let language_code = "en";
        sqlx::query_as("SELECT s.sku_id, st.sku_name, st.sku_description FROM sku s LEFT JOIN sku_translation st ON s.sku_id = st.sku_id AND st.language_code = $1 WHERE sku_id IN $2")
            .bind(language_code)
            .bind(ids)
            .fetch_all(&*DB_POOL)
            .await
    }
}
