use futures::future::BoxFuture;
use sea_orm::*;
use uuid::Uuid;
use crate::db::DB;
use crate::entities::media::Model as MediaModel;
use crate::entities::sku_translation::Model as SkuTranslationModel;
use crate::entities::{media, media_media_set, prelude::*, sku, sku_translation};

type DbFuture = BoxFuture<'static, Result<Box<dyn std::any::Any + Send>, DbErr>>;

// Object used in templates
pub struct SkuData {
    pub sku_id: Uuid,
    pub sku_fields: Option<SkuTranslationModel>,
    pub media_set_id: Option<Uuid>,
    pub media: Vec<MediaModel>
}

#[derive(Default)]
pub struct SkuBuilder {
    sku_id: Uuid,
    media_set_id: Option<Uuid>,
    sku_fields: Option<SkuTranslationModel>,
    media: Vec<MediaModel>,
    futures: Vec<DbFuture>
}

#[derive(Default)]
pub struct MultiSkuBuilder {
    sku_ids: Vec<Uuid>,
    media_set_ids: Vec<Option<Uuid>>
}

impl SkuBuilder {
    pub async fn new(sku_id: Uuid) -> Result<SkuBuilder, DbErr> {
        let sku_obj = Sku::find_by_id(sku_id).one(&*DB).await.unwrap();

        if let Some(sku) = sku_obj {
            Ok(SkuBuilder {
                sku_id,
                media_set_id: sku.media_set_id,
                ..Default::default()
            })
        } else {
            Err(DbErr::RecordNotFound("SKU not found by id!".into()))
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
        self.sku_fields = sku_translation::Entity::find().filter(sku_translation::Column::SkuId.eq(self.sku_id)).one(&*DB).await.unwrap();

        self
    }

    pub async fn add_all(mut self) -> Self {
        let media_future = media::Entity::find()
        .join(sea_orm::JoinType::LeftJoin, media::Relation::MediaMediaSet.def())
        .filter(media_media_set::Column::MediaSetId.eq(*&self.media_set_id.unwrap()))
        .all(&*DB);

        let fields_future = sku_translation::Entity::find().filter(sku_translation::Column::SkuId.eq(self.sku_id)).one(&*DB);

        let media_and_fields = futures::try_join!(media_future, fields_future);

        (self.media, self.sku_fields) = media_and_fields.unwrap();

        self
    }


    pub async fn build(self) -> SkuData {
        SkuData {
            sku_id: self.sku_id,
            sku_fields: self.sku_fields,
            media_set_id: self.media_set_id,
            media: self.media
        }
    }

}