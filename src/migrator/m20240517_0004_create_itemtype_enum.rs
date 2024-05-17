use sea_orm_migration::prelude::{sea_query::extension::postgres::Type, *};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240517_0004_create_itemtype_enum"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ItemType::Enum)
                    .values([ItemType::Product, ItemType::Category])
                    .to_owned(),
            )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(ItemType::Enum).to_owned())
        .await
    }
}

#[derive(DeriveIden)]
pub enum ItemType {
    #[sea_orm(iden = "itemtype")]
    Enum,
    #[sea_orm(iden = "Product")]
    Product,
    #[sea_orm(iden = "Category")]
    Category
}
