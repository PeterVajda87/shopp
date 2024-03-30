use crate::models::{category::Category, product::Product};
use crate::templates::category_page::CategoryPage;
use crate::DbPool;
use ntex::web::{
    types::{Path, State},
    HttpRequest, HttpResponse,
};
use uuid::Uuid;

pub async fn category_page(_req: HttpRequest, id: Path<Uuid>, pool: State<DbPool>) -> HttpResponse {
    let category: Category = Category::get(id.into_inner(), &pool).await;
    let category_products: Vec<Product> = category.get_products(&pool).await;

    HttpResponse::Ok().body(
        CategoryPage {
            title: &format!("Category {} page", category.title),
            products: &category_products,
        }
        .to_string(),
    )
}
