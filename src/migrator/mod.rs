use sea_orm_migration::prelude::*;
mod m20240517_0001_create_category_table;
mod m20240517_0002_create_product_table;
mod m20240517_0003_create_product_to_category_table;
mod m20240517_0004_create_itemtype_enum;
mod m20240517_0005_create_language_table;
mod m20240519_0006_create_mediatype_enum;
mod m20240519_0007_create_media_table;
mod m20240519_0008_create_media_to_item_table;
mod m20240522_0009_create_sku_table;
mod m20240522_0010_create_sku_to_product_table;
mod m20240522_0011_create_slug_table;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240517_0001_create_category_table::Migration),
            Box::new(m20240517_0002_create_product_table::Migration),
            Box::new(m20240517_0003_create_product_to_category_table::Migration),
            Box::new(m20240517_0004_create_itemtype_enum::Migration),
            Box::new(m20240517_0005_create_language_table::Migration),
            Box::new(m20240519_0006_create_mediatype_enum::Migration),
            Box::new(m20240519_0007_create_media_table::Migration),
            Box::new(m20240519_0008_create_media_to_item_table::Migration),
            Box::new(m20240522_0009_create_sku_table::Migration),
            Box::new(m20240522_0010_create_sku_to_product_table::Migration),
            Box::new(m20240522_0011_create_slug_table::Migration),
        ]
    }
}
