use async_trait::async_trait;
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

pub trait HasSkus {}
pub trait HasId {}
pub trait HasProducts {}
