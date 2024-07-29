//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sku")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: Option<DateTime>,
    pub media_set_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::media_set::Entity",
        from = "Column::MediaSetId",
        to = "super::media_set::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    MediaSet,
    #[sea_orm(has_many = "super::sku_product::Entity")]
    SkuProduct,
    #[sea_orm(has_many = "super::sku_translation::Entity")]
    SkuTranslation,
}

impl Related<super::media_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MediaSet.def()
    }
}

impl Related<super::sku_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SkuProduct.def()
    }
}

impl Related<super::sku_translation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SkuTranslation.def()
    }
}

impl Related<super::language::Entity> for Entity {
    fn to() -> RelationDef {
        super::sku_translation::Relation::Language.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::sku_translation::Relation::Sku.def().rev())
    }
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        super::sku_product::Relation::Product.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::sku_product::Relation::Sku.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
