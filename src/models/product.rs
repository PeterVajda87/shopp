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

pub struct ProductImage {
    pub product_image_id: Uuid,
    pub product_id: Uuid,
    pub image_order: i16,
    pub main_image: Option<bool>,
    pub path: Option<String>,
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

    pub async fn get_main_image(&self, pool: &State<DbPool>) -> Result<ProductImage, sqlx::Error> {
        sqlx::query_as!(
            ProductImage,
            r#"SELECT * FROM product_image WHERE product_id = $1 AND main_image = True"#,
            self.product_id
        )
        .fetch_one(pool.get_ref())
        .await
    }

    pub async fn get_all_images(
        &self,
        pool: &State<DbPool>,
    ) -> Result<Vec<ProductImage>, sqlx::Error> {
        sqlx::query_as!(
            ProductImage,
            r#"SELECT * FROM product_image WHERE product_id = $1"#,
            self.product_id
        )
        .fetch_all(pool.get_ref())
        .await
    }
}

impl Render for Product {
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.title.render(writer)
    }
}
