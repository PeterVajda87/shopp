use std::future::Future;

use async_std::future;
use futures::future::try_join_all;
use futures::TryFutureExt;
use sea_orm::*;
use uuid::Uuid;
use crate::db::DB;
use crate::entities::media::Model as MediaModel;
use crate::entities::product_translation::Model as ProductTranslationModel;
use crate::entities::{media, media_media_set, prelude::*, product_translation, sku_product};

use super::sku::{SkuBuilder, SkuData};

// Object used in templates
pub struct ProductData {
    pub product_id: Uuid,
    pub product_fields: Option<ProductTranslationModel>,
    pub media_set_id: Option<Uuid>,
    pub media: Vec<MediaModel>,
    pub skus: Vec<SkuData>
}

#[derive(Default)]
pub struct ProductBuilder {
    product_id: Uuid,
    media_set_id: Option<Uuid>,
    product_fields: Option<ProductTranslationModel>,
    media: Vec<MediaModel>,
    skus: Vec<SkuData>
}

impl ProductBuilder {
    pub async fn new(product_id: Uuid) -> Result<ProductBuilder, DbErr> {
        let product_obj: Option<crate::entities::product::Model> = Product::find_by_id(product_id).one(&*DB).await.unwrap();

        if let Some(product) = product_obj {
            Ok(ProductBuilder {
                product_id,
                media_set_id: product.media_set_id,
                ..Default::default()
            })
        } else {
            Err(DbErr::RecordNotFound("Product not found by id!".into()))
        }
    }

    pub async fn add_media(mut self) -> Self {
        self.media = media::Entity::find()
        .join(sea_orm::JoinType::LeftJoin, media::Relation::MediaMediaSet.def())
        .filter(media_media_set::Column::MediaSetId.eq(*&self.media_set_id.unwrap()))
        .all(&*DB)
        .await
        .unwrap();

        self
    }

    pub async fn add_fields(mut self) -> Self {
        self.product_fields = product_translation::Entity::find().filter(product_translation::Column::ProductId.eq(self.product_id)).one(&*DB).await.unwrap();

        self
    }

    pub async fn add_skus(mut self) -> Self {
        let sku_to_product_ids = sku_product::Entity::find().filter(sku_product::Column::ProductId.eq(*&self.product_id)).select_only().column(sku_product::Column::SkuId).all(&*DB).await.unwrap();
        let sku_ids: Vec<Uuid> = sku_to_product_ids.iter().map(|sku_to_product_id| sku_to_product_id.sku_id).collect();


           // Collect all futures
        let futures: Vec<_> = sku_ids.into_iter().map(|sku_id| {
            async move {
                SkuBuilder::new(sku_id).await.unwrap().add_all().await.build().await
            }
        }).collect();

        // Await all futures concurrently
        let results = futures::future::join_all(futures).await;
       
        self.skus.extend(results);
        
        self
    }

    // pub async fn add_all(mut self) -> Self {
    //     let media_and_fields = futures::try_join!(media::Entity::find()
    //     .join(sea_orm::JoinType::LeftJoin, media::Relation::MediaMediaSet.def())
    //     .filter(media_media_set::Column::MediaSetId.eq(*&self.media_set_id.unwrap()))
    //     .all(&*DB), sku_translation::Entity::find().filter(sku_translation::Column::SkuId.eq(self.sku_id)).one(&*DB));

    //     (self.media, self.sku_fields) = media_and_fields.unwrap();

    //     self
    // }


    pub async fn build(self) -> ProductData {
        ProductData {
            product_id: self.product_id,
            product_fields: self.product_fields,
            media_set_id: self.media_set_id,
            media: self.media,
            skus: self.skus
        }
    }

}