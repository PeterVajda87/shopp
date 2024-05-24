use crate::entities::{media, media_to_item, prelude::*, product, sku, sku_to_product};
use ntex::web::{
    types::{Path, State},
    HttpRequest, HttpResponse,
};
use sea_orm::*;
use uuid::Uuid;

pub struct SkuWithData {
    pub sku: sku::Model,
    pub media: Option<Vec<media::Model>>
}

pub struct ProductWithData {
    pub skus: Vec<SkuWithData>,
    pub product: product::Model,
    pub media: Option<Vec<media::Model>>
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
        let product_skus = SkuToProduct::find()
            .filter(sku_to_product::Column::ProductId.eq(product.id))
            .find_with_related(Sku)
            .all(&*conn)
            .await
            .unwrap()
            .into_iter()
            .flat_map(|(_, skus)| skus.into_iter())
            .collect::<Vec<sku::Model>>();

        let mut skus_with_data = Vec::new();
        for sku in product_skus {
            skus_with_data.push(SkuWithData {
                sku,
                media: None,
            })
        }

        let product_with_data = ProductWithData {
            product,
            skus: skus_with_data,
            media: None
        }

        let product_media = MediaToItem::find()
            .filter(media_to_item::Column::ItemId.eq(product.id))
            .find_with_related(Media)
            .all(&*conn)
            .await
            .unwrap();
        HttpResponse::Ok().body(
            crate::templates::product::ProductPage {
                title: &format!("Product {} page", product.name),
                product,
            }
            .to_string(),
        )
    }
}
