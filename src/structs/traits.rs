use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

#[async_trait]
pub trait AddSkus {
    async fn add_skus(mut self) -> Result<Self, sqlx::Error>
    where
        Self: Sized + HasSkus;
}

#[async_trait]
pub trait AddProducts {
    async fn add_products(mut self) -> Result<Self, sqlx::Error>
    where
        Self: Sized + HasProducts;
}

#[async_trait]
pub trait GetById {
    async fn get_by_id(id: Uuid) -> Result<Self, sqlx::Error>
    where
        Self: Sized + HasId;
}

#[async_trait]
pub trait FilterById {
    async fn filter_by_id(ids: Vec<Uuid>) -> Result<Self, sqlx::Error>
    where
        Self: Sized + HasId;
}

#[async_trait]
pub trait FromRequest<T> {
    async fn create_from_request(
        json_request: ntex::web::types::Json<T>,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized;
}

#[async_trait]
pub trait Storable {
    async fn insert(self) -> Result<PgQueryResult, sqlx::Error>
    where
        Self: Sized;
}

pub trait HasSkus {}
pub trait HasId {}
pub trait HasProducts {}
