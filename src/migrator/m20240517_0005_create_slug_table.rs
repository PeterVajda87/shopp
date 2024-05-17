use sea_orm_migration::prelude::*;
use super::m20240517_0004_create_itemtype_enum::ItemType;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240517_0005_create_slug_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Slug::Table)
                    .index(Index::create()
                            .name("idx-slug-item_id")
                            .table(Slug::Table)
                            .col(Slug::Slug)
                            .col(Slug::ItemId)
                            .unique()
                            .nulls_not_distinct())
                    .col(
                        ColumnDef::new(Slug::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::gen_random_uuid())
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Slug::Slug).string().not_null())
                    .col(ColumnDef::new(Slug::ItemId).uuid().not_null())
                    .col(ColumnDef::new(Slug::ItemType).not_null().enumeration(ItemType::Enum, [ItemType::Product, ItemType::Category]))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Slug::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Slug {
    Table,
    Id,
    Slug,
    ItemId,
    ItemType
}