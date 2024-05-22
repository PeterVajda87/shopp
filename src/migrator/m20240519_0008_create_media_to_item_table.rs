use super::m20240517_0004_create_itemtype_enum::ItemType;
use super::m20240519_0007_create_media_table::Media;

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240519_0008_create_media_to_item_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MediaToItem::Table)
                    .index(
                        Index::create()
                            .name("idx-media-to-item")
                            .table(MediaToItem::Table)
                            .col(MediaToItem::MediaId)
                            .col(MediaToItem::ItemId)
                            .unique(),
                    )
                    .col(
                        ColumnDef::new(MediaToItem::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::gen_random_uuid())
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MediaToItem::MediaId).uuid().not_null())
                    .col(ColumnDef::new(MediaToItem::ItemId).uuid().not_null())
                    .col(ColumnDef::new(MediaToItem::Order).small_unsigned().not_null())
                    .col(
                        ColumnDef::new(MediaToItem::ItemType)
                            .not_null()
                            .enumeration(
                                ItemType::Enum,
                                [ItemType::Product, ItemType::Category, ItemType::SKU, ItemType::Page],
                            ),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-media")
                            .from(MediaToItem::Table, MediaToItem::MediaId)
                            .to(Media::Table, Media::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MediaToItem::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum MediaToItem {
    Table,
    Id,
    MediaId,
    ItemId,
    ItemType,
    Order,
}
