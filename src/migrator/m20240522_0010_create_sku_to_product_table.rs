use super::m20240517_0002_create_product_table::Product;
use super::m20240522_0009_create_sku_table::SKU;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240522_0009_create_sku_to_product_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SKUToProduct::Table)
                    .index(
                        Index::create()
                            .name("idx-SKU-to-product")
                            .table(SKUToProduct::Table)
                            .col(SKUToProduct::ProductId)
                            .col(SKUToProduct::SKUId)
                            .unique(),
                    )
                    .col(
                        ColumnDef::new(SKUToProduct::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::gen_random_uuid())
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SKUToProduct::ProductId).uuid().not_null())
                    .col(ColumnDef::new(SKUToProduct::SKUId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product")
                            .from(SKUToProduct::Table, SKUToProduct::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-SKU")
                            .from(SKUToProduct::Table, SKUToProduct::SKUId)
                            .to(SKU::Table, SKU::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SKUToProduct::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum SKUToProduct {
    Table,
    Id,
    SKUId,
    ProductId,
}
