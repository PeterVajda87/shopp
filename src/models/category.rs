use crate::models::product::Product;
use crate::DbPool;
use ntex::web::types::State;
use uuid::Uuid;

#[allow(dead_code)]
pub struct Category {
    pub category_id: Uuid,
    pub title: String,
    pub parent_category_id: Option<Uuid>,
}

impl Category {
    pub async fn get(id: Uuid, pool: &State<DbPool>) -> Category {
        sqlx::query_as!(
            Category,
            r#"SELECT * FROM category WHERE category_id = $1"#,
            id.clone()
        )
        .fetch_one(pool.get_ref())
        .await
        .unwrap()
    }

    pub async fn get_products(&self, pool: &State<DbPool>) -> Vec<Product> {
        sqlx::query_as!(
            Product,
            r#"SELECT * FROM product WHERE category_id = $1"#,
            &self.category_id.clone()
        )
        .fetch_all(pool.get_ref())
        .await
        .unwrap()
    }
}
