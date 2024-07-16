use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveEnum, ActiveValue, IntoSimpleExpr, Value};
use sea_orm_migration::prelude::sea_query::extension::postgres::Type;
use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT NOW()"),
                    )
                    .to_owned(),
            )
            .await?;

        let product_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));

        let insert_product: InsertStatement = Query::insert()
            .into_table(Product::Table)
            .columns([Product::Name, Product::Id])
            .values_panic([
                "Skoda 105".into(),
                SimpleExpr::Value(Value::Uuid(product_uuid.clone())),
            ])
            .returning_col(Product::Id)
            .to_owned();

        manager.exec_stmt(insert_product).await.unwrap();

        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .col(
                        ColumnDef::new(Category::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Category::ParentCataegory).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Category::Table, Category::ParentCataegory)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let category_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));

        let insert_category: InsertStatement = Query::insert()
            .into_table(Category::Table)
            .columns([Category::Name, Category::Id])
            .values_panic([
                "Cars".into(),
                SimpleExpr::Value(Value::Uuid(category_uuid.clone())),
            ])
            .returning_all()
            .to_owned();

        manager.exec_stmt(insert_category).await?;

        manager
            .create_table(
                Table::create()
                    .table(ProductCategory::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProductCategory::ProductId).uuid().not_null())
                    .col(
                        ColumnDef::new(ProductCategory::CategoryId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .table(ProductCategory::Table)
                            .col(ProductCategory::ProductId)
                            .col(ProductCategory::CategoryId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductCategory::Table, ProductCategory::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductCategory::Table, ProductCategory::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let insert_product_category: InsertStatement = Query::insert()
            .into_table(ProductCategory::Table)
            .columns([ProductCategory::ProductId, ProductCategory::CategoryId])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(product_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(category_uuid.clone())),
            ])
            .returning_all()
            .to_owned();

        manager.exec_stmt(insert_product_category).await?;

        manager
            .create_table(
                Table::create()
                    .table(Sku::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sku::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(Sku::Name).string().not_null())
                    .col(
                        ColumnDef::new(Sku::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SkuProduct::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SkuProduct::SkuId).uuid().not_null())
                    .col(ColumnDef::new(SkuProduct::ProductId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .table(SkuProduct::Table)
                            .col(SkuProduct::SkuId)
                            .col(SkuProduct::ProductId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SkuProduct::Table, SkuProduct::SkuId)
                            .to(Sku::Table, Sku::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SkuProduct::Table, SkuProduct::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(EntityType::Enum)
                    .values([EntityType::Sku, EntityType::Product, EntityType::Category])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Slug::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Slug::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(
                        ColumnDef::new(Slug::EntityType)
                            .custom(EntityType::Enum)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Slug::EntityId).uuid().not_null())
                    .col(ColumnDef::new(Slug::Text).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-slug_text")
                    .table(Slug::Table)
                    .col(Slug::Text)
                    .unique()
                    .to_owned(),
            )
            .await?;

        let slug_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));

        let insert_slug: InsertStatement = Query::insert()
            .into_table(Slug::Table)
            .columns([Slug::Id, Slug::Text, Slug::EntityType, Slug::EntityId])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(slug_uuid)),
                "Auta".into(),
                Expr::val("Category").as_enum(Alias::new("entity_type")),
                SimpleExpr::Value(Value::Uuid(category_uuid)),
            ])
            .returning_all()
            .to_owned();

        manager.exec_stmt(insert_slug).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    Name,
    ParentCataegory,
    CreatedAt,
}

#[derive(DeriveIden)]
enum ProductCategory {
    Table,
    ProductId,
    CategoryId,
}

#[derive(DeriveIden)]
enum Sku {
    Table,
    Id,
    Name,
    CreatedAt,
}

#[derive(DeriveIden)]
enum SkuProduct {
    Table,
    SkuId,
    ProductId,
}

#[derive(DeriveIden)]
enum Slug {
    Table,
    Id,
    EntityId,
    EntityType,
    Text,
}

#[derive(DeriveIden)]
pub enum EntityType {
    #[sea_orm(iden = "entity_type")]
    Enum,
    #[sea_orm(iden = "Sku")]
    Sku,
    #[sea_orm(iden = "Product")]
    Product,
    #[sea_orm(iden = "Category")]
    Category,
}
