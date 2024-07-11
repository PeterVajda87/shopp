use crate::db::DB;
use crate::structs::Product;
use ntex::web::{types::Path, HttpRequest, HttpResponse};
use uuid::Uuid;

#[derive(Debug)]
pub struct ProductWithData {
    pub product: Product,
}

pub async fn product_page(_req: HttpRequest, product_id: Path<Uuid>) -> HttpResponse {
    let product_data: Option<ProductWithData> = get_product_data(*product_id).await;

    println!("{:?}", product_data);

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

pub async fn get_product_data(product_id: Uuid) -> Option<ProductWithData> {
    let product = sqlx::query_as!(
        Product,
        r#"SELECT * FROM product WHERE id = $1"#,
        product_id
    )
    .fetch_one(&*DB)
    .await;

    if let Ok(product) = product {
        Some(ProductWithData { product })
    } else {
        None
    }
}
