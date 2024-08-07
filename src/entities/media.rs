//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::MediaRole;
use super::sea_orm_active_enums::MediaType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "media")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub media_type: MediaType,
    pub media_role: MediaRole,
    pub path: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::media_media_set::Entity")]
    MediaMediaSet,
}

impl Related<super::media_media_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MediaMediaSet.def()
    }
}

impl Related<super::media_set::Entity> for Entity {
    fn to() -> RelationDef {
        super::media_media_set::Relation::MediaSet.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::media_media_set::Relation::Media.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
