//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sku")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub product_id: Uuid,
    pub gallery_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::gallery::Entity",
        from = "Column::GalleryId",
        to = "super::gallery::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Gallery,
    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::ProductId",
        to = "super::product::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Product,
}

impl Related<super::gallery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gallery.def()
    }
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
