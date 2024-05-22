use crate::entities::{prelude::Product, product};
use ntex::web::{types::{Path, State},
    HttpRequest, HttpResponse,
};
use sea_orm::*;
use uuid::Uuid;


pub async fn product_page(
    _req: HttpRequest,
    id: Path<Uuid>,
    conn: State<DatabaseConnection>,
) -> HttpResponse {
    let product_opt: Option<product::Model> = Product::find()
        .filter(product::Column::Id.eq(*id))
        .one(&*conn)
        .await
        .expect("Failed to execute search query");

    if let Some(product) = product_opt {
        HttpResponse::Ok().body(
            crate::templates::product::ProductPage {
                title: &format!("Product {} page", product.name),
                product,
            }
            .to_string(),
        )
    } else {
        HttpResponse::from("produkt som nenasiel".to_string())
    }
}
