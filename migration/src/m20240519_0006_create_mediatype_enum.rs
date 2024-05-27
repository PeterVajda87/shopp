use sea_orm_migration::prelude::{sea_query::extension::postgres::Type, *};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240519_0006_create_mediatype_enum"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(MediaType::Enum)
                    .values([
                        MediaType::Image,
                        MediaType::Video,
                        MediaType::Audio,
                        MediaType::File,
                    ])
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(MediaType::Enum).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MediaType {
    #[sea_orm(iden = "mediatype")]
    Enum,
    #[sea_orm(iden = "Image")]
    Image,
    #[sea_orm(iden = "Video")]
    Video,
    #[sea_orm(iden = "Audio")]
    Audio,
    #[sea_orm(iden = "File")]
    File,
}
