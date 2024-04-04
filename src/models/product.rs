use crate::DbPool;
use markup::Render;
use ntex::web::types::State;
use uuid::Uuid;

#[allow(dead_code)]
pub struct Product {
    pub product_id: Uuid,
    pub title: String,
    pub category_id: Option<Uuid>,
}

impl Product {
    pub async fn get(id: Uuid, pool: &State<DbPool>) -> Result<Product, sqlx::Error> {
        sqlx::query_as!(
            Product,
            r#"SELECT * FROM product WHERE product_id = $1"#,
            id.clone()
        )
        .fetch_one(pool.get_ref())
        .await
    }
}

impl Render for Product {
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.title.render(writer)
    }
}
