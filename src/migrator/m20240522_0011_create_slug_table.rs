use super::m20240517_0004_create_itemtype_enum::ItemType;
use super::m20240517_0005_create_language_table::Language;
use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Slug::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::gen_random_uuid())
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Slug::ItemId).uuid().not_null())
                    .col(ColumnDef::new(Slug::LanguageId).uuid().not_null())
                    .col(ColumnDef::new(Slug::Text).string().not_null())
                    .col(ColumnDef::new(Slug::ItemType).not_null().enumeration(
                        ItemType::Enum,
                        [
                            ItemType::Product,
                            ItemType::Category,
                            ItemType::SKU,
                            ItemType::Page,
                        ],
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-slug-language")
                            .from(Slug::Table, Slug::LanguageId)
                            .to(Language::Table, Language::Id),
                    )
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
    LanguageId,
    Text,
    ItemId,
    ItemType,
}
