use sea_orm_migration::prelude::*;
use super::m20240517_0002_create_product_table::Product;
use super::m20240517_0001_create_category_table::Category;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240517_0003_create_product_to_category_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductToCategory::Table)
                    .index(Index::create()
                        .name("idx-product-to-category")
                        .table(ProductToCategory::Table)
                        .col(ProductToCategory::ProductId)
                        .col(ProductToCategory::CategoryId)
                        .unique())
                    .col(
                        ColumnDef::new(ProductToCategory::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductToCategory::ProductId).uuid().not_null())
                    .col(ColumnDef::new(ProductToCategory::CategoryId).uuid().not_null())
                    .foreign_key(ForeignKey::create()
                                    .name("fk-product")
                                    .from(ProductToCategory::Table, ProductToCategory::ProductId)
                                    .to(Product::Table, Product::Id))
                    .foreign_key(ForeignKey::create()
                                    .name("fk-category")
                                    .from(ProductToCategory::Table, ProductToCategory::CategoryId)
                                    .to(Category::Table, Category::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ProductToCategory{
    Table,
    Id,
    ProductId,
    CategoryId
}
