use crate::db::DB;
use crate::structs::{Description, Product};
use ntex::web::{types::Path, HttpRequest, HttpResponse};
use uuid::Uuid;

#[derive(Debug)]
pub struct ProductData {
    pub name: String,
    pub description: Description
}

pub async fn product_page(_req: HttpRequest, product_id: Path<Uuid>) -> HttpResponse {
    let product_data: Option<ProductData> = get_product_data(*product_id).await;

    if let Some(product) = product_data {
        HttpResponse::Ok().body(
            crate::templates::product::ProductPage {
                title: &format!("Product page"),
                product_data: product,
            }
            .to_string(),
        )
    } else {
        HttpResponse::Ok().body("ABC".to_string())
    }
}

pub async fn get_product_data(product_id: Uuid) -> Option<ProductData> {
    let product = sqlx::query_as!(
        ProductData,
        r#"SELECT p.name, (d.id, d.text, d.entity_id, d.language_id) as "description!: Description" FROM product p LEFT JOIN entity e ON e.entity_id = p.id
        LEFT JOIN description d ON d.entity_id = e.entity_id
        WHERE p.id = $1"#,
        product_id
    )
    .fetch_one(&*DB)
    .await;

    println!("{:?}", product);

    if let Ok(product) = product {
        Some(product)
    } else {
        None
    }
}