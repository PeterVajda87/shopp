use crate::models::{category::Category, product::Product};
use crate::templates::category_page::CategoryPage;
use crate::DbPool;
use ntex::web::{
    self,
    types::{Path, State},
    HttpRequest,
};
use uuid::Uuid;

pub async fn category_page(
    _req: HttpRequest,
    id: Path<Uuid>,
    pool: State<DbPool>,
) -> web::HttpResponse {
    let id: Uuid = id.into_inner();
    let category = sqlx::query_as!(
        Category,
        r#"SELECT * FROM category WHERE category_id = $1"#,
        id.clone()
    )
    .fetch_one(&*pool)
    .await
    .expect("Non existing category, TODO");

    let category_products = sqlx::query_as!(
        Product,
        r#"SELECT * FROM product WHERE category_id = $1"#,
        id.clone()
    )
    .fetch_all(&*pool)
    .await
    .expect("Empty category, TODO");

    web::HttpResponse::Ok().body(
        CategoryPage {
            title: &format!("Category {} page", category.title),
            products: &category_products,
        }
        .to_string(),
    )
}
