use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240517_0001_create_category_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .index(Index::create()
                            .name("idx-category-parent")
                            .table(Category::Table)
                            .col(Category::Name)
                            .col(Category::ParentCategoryId)
                            .unique()
                            .nulls_not_distinct())
                    .col(
                        ColumnDef::new(Category::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::gen_random_uuid())
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .col(ColumnDef::new(Category::ParentCategoryId).uuid().null())
                    .foreign_key(ForeignKey::create()
                                    .name("fk-category-parent_category")
                                    .from(Category::Table, Category::ParentCategoryId)
                                    .to(Category::Table, Category::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Category {
    Table,
    Id,
    Name,
    ParentCategoryId
}
