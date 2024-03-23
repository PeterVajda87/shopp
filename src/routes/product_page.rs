use crate::models::product::Product;
use crate::templates::product_page::ProductPage;
use crate::DbPool;
use ntex::web::{
    types::{Path, State},
    HttpRequest, HttpResponse,
};
use uuid::Uuid;

pub async fn product_page(_req: HttpRequest, id: Path<Uuid>, pool: State<DbPool>) -> HttpResponse {
    let product = sqlx::query_as!(
        Product,
        r#"SELECT * FROM product WHERE product_id = $1"#,
        id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await
    .expect("Non existing product, TODO");

    HttpResponse::Ok().body(
        ProductPage {
            title: &format!("Product {} page", product.title),
        }
        .to_string(),
    )
}
