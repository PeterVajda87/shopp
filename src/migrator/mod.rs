use sea_orm_migration::prelude::*;
mod m20240517_0001_create_category_table;
mod m20240517_0002_create_product_table;
mod m20240517_0003_create_product_to_category_table;
mod m20240517_0004_create_itemtype_enum;
mod m20240517_0005_create_slug_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240517_0001_create_category_table::Migration),
            Box::new(m20240517_0002_create_product_table::Migration),
            Box::new(m20240517_0003_create_product_to_category_table::Migration),
            Box::new(m20240517_0004_create_itemtype_enum::Migration),
            Box::new(m20240517_0005_create_slug_table::Migration),
        ]
    }
}