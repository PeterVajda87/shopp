use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240522_0009_create_SKU_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SKU::Table)
                    .col(
                        ColumnDef::new(SKU::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(SKU::Name).string().not_null())
                    .col(ColumnDef::new(SKU::EAN).string().unique_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SKU::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum SKU {
    Table,
    Id,
    Name,
    EAN
}
