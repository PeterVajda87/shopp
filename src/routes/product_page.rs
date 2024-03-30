use crate::models::product::Product;
use crate::templates::product_page::ProductPage;
use crate::DbPool;
use ntex::web::{
    types::{Path, State},
    HttpRequest, HttpResponse,
};
use uuid::Uuid;

pub async fn product_page(_req: HttpRequest, id: Path<Uuid>, pool: State<DbPool>) -> HttpResponse {
    let product: Product = Product::get(id.into_inner(), &pool).await;

    HttpResponse::Ok().body(
        ProductPage {
            title: &format!("Product {} page", product.title),
        }
        .to_string(),
    )
}
