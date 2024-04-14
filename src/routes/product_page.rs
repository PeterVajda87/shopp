use crate::models::product::Product;
use crate::templates::product_page::ProductPage;
use crate::DbPool;
use ntex::web::{
    types::{Path, State},
    HttpRequest, HttpResponse,
};
use uuid::Uuid;

use super::not_found_page::not_found_page;

pub async fn product_page(req: HttpRequest, id: Path<Uuid>, pool: State<DbPool>) -> HttpResponse {
    if let Ok(product) = Product::get(id.into_inner(), &pool).await {
        HttpResponse::Ok().body(
            ProductPage {
                title: &format!("Product {} page", product.title),
                product_image: product
                    .get_main_image(&pool)
                    .await
                    .expect("Failed to fetch product image"),
                product,
            }
            .to_string(),
        )
    } else {
        not_found_page(req).await
    }
}
