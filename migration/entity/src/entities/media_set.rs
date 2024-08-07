//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "media_set")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
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

impl Related<super::media::Entity> for Entity {
    fn to() -> RelationDef {
        super::media_media_set::Relation::Media.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::media_media_set::Relation::MediaSet.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
