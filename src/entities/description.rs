//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::EntityType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "description")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub entity_type: EntityType,
    pub entity_id: Uuid,
    pub language_code: String,
    pub text: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::language::Entity",
        from = "Column::LanguageCode",
        to = "super::language::Column::Code",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Language,
}

impl Related<super::language::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Language.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
