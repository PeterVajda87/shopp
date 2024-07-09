use crate::entities::{
    media_item, media_set, prelude::*, product, sea_orm_active_enums::*, sku,
};
use ntex::web::{
    types::Path, HttpRequest, HttpResponse
};
use sea_orm::*;
use uuid::Uuid;
use crate::DUMMY_UUID;
use crate::db::DB;

#[derive(Debug)]
pub struct ProductWithData {
    pub product: product::Model,
    pub gallery: Vec<media_item::Model>,
}

pub async fn product_page(_req: HttpRequest, id: Path<Uuid>) -> HttpResponse {
    let product_data: Option<ProductWithData> = get_product_data(*id).await;

    println!("{:?}", &product_data);

    if let Some(product) = product_data {
        HttpResponse::Ok().body(
            crate::templates::product::ProductPage {
            title: &format!("Product {} page", &product.product.name),
            product_data: product,
        }
        .to_string(),
        )
    } else {
        HttpResponse::Ok().body("ABC".to_string())
    }
}

pub async fn get_product_data(product_id: Uuid) -> Option<ProductWithData> {
    let product_opt: Option<product::Model> = Product::find()
        .filter(product::Column::Id.eq(product_id))
        .one(&*DB)
        .await
        .expect("Failed to execute search query");

    if let None = product_opt {
        None
    } else {
        let product = product_opt.unwrap();

        let product_media_set = MediaSet::find()
            .filter(media_set::Column::Id.eq(product.media_set_id.unwrap_or(*DUMMY_UUID)))
            .one(&*DB)
            .await
            .expect("Failed to fetch media set")
            .unwrap();

        let gallery = MediaItem::find()
            .filter(media_item::Column::MediaSetId.eq(product_media_set.id))
            .filter(media_item::Column::Role.eq(MediaRole::Gallery))
            .all(&*DB)
            .await
            .expect("Failed to fetch product gallery media");

        let _product_skus = Product::find()
            .find_also_related(Sku)
            .all(&*DB)
            .await
            .unwrap()
            .into_iter()
            .flat_map(|(_, opt_sku)| opt_sku)
            .collect::<Vec<sku::Model>>();

        let product_with_data = ProductWithData {
            product,
            gallery
        };
        
        Some(product_with_data)
    }

}
