use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240517_0005_create_language_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Language::Table)
                    .col(
                        ColumnDef::new(Language::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::gen_random_uuid())
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Language::LanguageCode)
                            .string_len(2)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Language::Name)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Language::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Language {
    Table,
    Id,
    LanguageCode,
    Name,
}
