//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::product_category::Entity")]
    ProductCategory,
    #[sea_orm(has_many = "super::sku_product::Entity")]
    SkuProduct,
}

impl Related<super::product_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductCategory.def()
    }
}

impl Related<super::sku_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SkuProduct.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        super::product_category::Relation::Category.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::product_category::Relation::Product.def().rev())
    }
}

impl Related<super::sku::Entity> for Entity {
    fn to() -> RelationDef {
        super::sku_product::Relation::Sku.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::sku_product::Relation::Product.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
