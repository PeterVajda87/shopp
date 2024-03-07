use crate::templates::product_page::ProductPage;
use crate::DbPool;
use ntex::web::{self, types, HttpRequest};
use uuid::Uuid;

struct Product {
    id: Uuid,
    title: String,
}

pub async fn product_page(
    _req: HttpRequest,
    id: types::Path<Uuid>,
    pool: types::State<DbPool>,
) -> web::HttpResponse {
    println!("Id: {:?}", &id);
    let product = sqlx::query_as!(
        Product,
        r#"SELECT id, title FROM product WHERE id = $1"#,
        id.into_inner()
    )
    .fetch_one(&*pool)
    .await
    .expect("Non existing product, TODO");

    web::HttpResponse::Ok().body(
        ProductPage {
            title: &format!("Product {} page", product.title),
        }
        .to_string(),
    )
}
