use crate::entities::{media, prelude::*, product, sku};
use ntex::web::{
    types::{Path, State},
    HttpRequest, HttpResponse,
};
use sea_orm::*;
use uuid::Uuid;

pub struct SkuWithData {
    pub sku: sku::Model,
}

pub struct ProductWithData {
    pub skus: Vec<SkuWithData>,
    pub product: product::Model,
}

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

    if let None = product_opt {
        HttpResponse::from("produkt som nenasiel".to_string())
    } else {
        let product = product_opt.unwrap();
        let product_skus = Product::find()
            .find_also_related(Sku)
            .all(&*conn)
            .await
            .unwrap()
            .into_iter()
            .flat_map(|(_, skus)| skus.into_iter())
            .collect::<Vec<sku::Model>>();

        let mut skus_with_data = Vec::new();
        for sku in product_skus {
            skus_with_data.push(SkuWithData { sku })
        }

        let product_with_data = ProductWithData {
            product: product.clone(),
            skus: vec![],
        };

        HttpResponse::Ok().body(
            crate::templates::product::ProductPage {
                title: &format!("Product {} page", product.name),
                product_data: product_with_data,
            }
            .to_string(),
        )
    }
}
