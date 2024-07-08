//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::MediaRole;
use super::sea_orm_active_enums::MediaType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "media_item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub path: String,
    pub r#type: MediaType,
    pub role: MediaRole,
    pub media_set_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::media_set::Entity",
        from = "Column::MediaSetId",
        to = "super::media_set::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    MediaSet,
}

impl Related<super::media_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MediaSet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
