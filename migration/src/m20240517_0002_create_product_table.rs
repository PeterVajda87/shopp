use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240517_0002_create_product_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .col(
                        ColumnDef::new(Product::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
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
pub enum Product {
    Table,
    Id,
    Name,
}
