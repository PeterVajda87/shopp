use super::m20240519_0006_create_mediatype_enum::MediaType;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240519_0007_create_media_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Media::Table)
                    .col(
                        ColumnDef::new(Media::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(Media::Path).string().not_null().unique_key())
                    .col(ColumnDef::new(Media::MediaType).not_null().enumeration(
                        MediaType::Enum,
                        [
                            MediaType::Video,
                            MediaType::Audio,
                            MediaType::Image,
                            MediaType::File,
                        ],
                    ))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Media::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Media {
    Table,
    Id,
    Path,
    MediaType,
}
