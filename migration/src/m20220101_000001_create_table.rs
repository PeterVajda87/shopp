use sea_orm::Value;
use sea_orm_migration::prelude::sea_query::extension::postgres::Type;
use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create table language
        manager
            .create_table(
                Table::create()
                    .table(Language::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Language::Id)
                            .uuid()
                            .not_null()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(Language::Code).not_null().string_len(2))
                    .col(ColumnDef::new(Language::Name).not_null().string())
                    .to_owned(),
            )
            .await?;

        // Create table product
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

        // Create table category
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

        // Create table product_category
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

        // Create table sku
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

        // Create table sku_product
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

        // Create type EntityType (sku, product, category)
        manager
            .create_type(
                Type::create()
                    .as_enum(EntityType::Enum)
                    .values([EntityType::Sku, EntityType::Product, EntityType::Category])
                    .to_owned(),
            )
            .await?;

        // Create type MediaType (image, video, document)
        manager
            .create_type(
                Type::create()
                    .as_enum(MediaType::Enum)
                    .values([MediaType::Image, MediaType::Video, MediaType::Document])
                    .to_owned(),
            )
            .await?;

        // Create type MediaRole (image, video, document)
        manager
            .create_type(
                Type::create()
                    .as_enum(MediaRole::Enum)
                    .values([
                        MediaRole::Attachment,
                        MediaRole::Gallery,
                        MediaRole::Description,
                    ])
                    .to_owned(),
            )
            .await?;

        // Create table description
        manager
            .create_table(
                Table::create()
                    .table(Description::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Description::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(Description::Text).string())
                    .col(ColumnDef::new(Description::LanguageId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Description::Table, Description::LanguageId)
                            .to(Language::Table, Language::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create table sku_description
        manager
            .create_table(
                Table::create()
                    .table(SkuDescription::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SkuDescription::SkuId).uuid().not_null())
                    .col(
                        ColumnDef::new(SkuDescription::DescriptionId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .table(SkuDescription::Table)
                            .col(SkuDescription::SkuId)
                            .col(SkuDescription::DescriptionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SkuDescription::Table, SkuDescription::SkuId)
                            .to(Sku::Table, Sku::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SkuDescription::Table, SkuDescription::DescriptionId)
                            .to(Description::Table, Description::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProductDescription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductDescription::ProductId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductDescription::DescriptionId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .table(ProductDescription::Table)
                            .col(ProductDescription::ProductId)
                            .col(ProductDescription::DescriptionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductDescription::Table, ProductDescription::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductDescription::Table, ProductDescription::DescriptionId)
                            .to(Description::Table, Description::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CategoryDescription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CategoryDescription::CategoryId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryDescription::DescriptionId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .table(CategoryDescription::Table)
                            .col(CategoryDescription::CategoryId)
                            .col(CategoryDescription::DescriptionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CategoryDescription::Table, CategoryDescription::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                CategoryDescription::Table,
                                CategoryDescription::DescriptionId,
                            )
                            .to(Description::Table, Description::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
                    .col(ColumnDef::new(Slug::Text).string().not_null())
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

        manager
            .create_table(
                Table::create()
                    .table(Media::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Media::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(
                        ColumnDef::new(Media::MediaType)
                            .custom(MediaType::Enum)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Media::MediaRole)
                            .custom(MediaRole::Enum)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Media::Path).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MediaSet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MediaSet::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MediaMediaSet::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MediaMediaSet::MediaId).uuid().not_null())
                    .col(ColumnDef::new(MediaMediaSet::MediaSetId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .table(MediaMediaSet::Table)
                            .col(MediaMediaSet::MediaId)
                            .col(MediaMediaSet::MediaSetId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MediaMediaSet::Table, MediaMediaSet::MediaId)
                            .to(Media::Table, Media::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MediaMediaSet::Table, MediaMediaSet::MediaSetId)
                            .to(MediaSet::Table, MediaSet::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MediaSetEntity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MediaSetEntity::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(MediaSetEntity::MediaSetId).uuid().not_null())
                    .col(ColumnDef::new(MediaSetEntity::EntityId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .table(MediaSetEntity::Table)
                            .col(MediaSetEntity::MediaSetId)
                            .col(MediaSetEntity::EntityId),
                    )
                    .col(
                        ColumnDef::new(MediaSetEntity::EntityType)
                            .custom(EntityType::Enum)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create UUIDs for sample data
        let language_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let language_uuid_2: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let product_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let product_uuid_2: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let category_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let sku_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let sku_uuid_2: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let slug_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let description_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));

        // Create insert queries
        let insert_language: InsertStatement = Query::insert()
            .into_table(Language::Table)
            .columns([Language::Id, Language::Code, Language::Name])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(language_uuid.clone())),
                "en".into(),
                "English".into(),
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(language_uuid_2.clone())),
                "cz".into(),
                "Česky".into(),
            ])
            .to_owned();

        let insert_product: InsertStatement = Query::insert()
            .into_table(Product::Table)
            .columns([Product::Name, Product::Id])
            .values_panic([
                "Skoda 120".into(),
                SimpleExpr::Value(Value::Uuid(product_uuid.clone())),
            ])
            .values_panic([
                "Skoda 105".into(),
                SimpleExpr::Value(Value::Uuid(product_uuid_2.clone())),
            ])
            .to_owned();

        let insert_category: InsertStatement = Query::insert()
            .into_table(Category::Table)
            .columns([Category::Name, Category::Id])
            .values_panic([
                "Cars".into(),
                SimpleExpr::Value(Value::Uuid(category_uuid.clone())),
            ])
            .to_owned();

        let insert_product_category: InsertStatement = Query::insert()
            .into_table(ProductCategory::Table)
            .columns([ProductCategory::ProductId, ProductCategory::CategoryId])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(product_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(category_uuid.clone())),
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(product_uuid_2.clone())),
                SimpleExpr::Value(Value::Uuid(category_uuid.clone())),
            ])
            .to_owned();

        let insert_sku: InsertStatement = Query::insert()
            .into_table(Sku::Table)
            .columns([Sku::Id, Sku::Name])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(sku_uuid.clone())),
                "Family Sedan".into(),
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(sku_uuid_2.clone())),
                "Sport Coupe".into(),
            ])
            .to_owned();

        let insert_sku_product: InsertStatement = Query::insert()
            .into_table(SkuProduct::Table)
            .columns([SkuProduct::SkuId, SkuProduct::ProductId])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(sku_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(product_uuid.clone())),
            ])
            .to_owned();

        let insert_slug: InsertStatement = Query::insert()
            .into_table(Slug::Table)
            .columns([Slug::Id, Slug::Text, Slug::EntityType, Slug::EntityId])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(slug_uuid)),
                "Auta".into(),
                Expr::val("Product").as_enum(Alias::new("entity_type")),
                SimpleExpr::Value(Value::Uuid(product_uuid.clone())),
            ])
            .to_owned();

        let insert_description: InsertStatement = Query::insert()
            .into_table(Description::Table)
            .columns([Description::Id, Description::Text, Description::LanguageId])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(description_uuid.clone())),
                "Hello, I am an English description".into(),
                SimpleExpr::Value(Value::Uuid(language_uuid.clone())),
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(description_uuid.clone())),
                "A ja jsem zas cesky popis".into(),
                SimpleExpr::Value(Value::Uuid(language_uuid_2.clone())),
            ])
            .to_owned();

        let insert_product_description = Query::insert()
            .into_table(ProductDescription::Table)
            .columns([
                ProductDescription::ProductId,
                ProductDescription::DescriptionId,
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(product_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(description_uuid.clone())),
            ])
            .to_owned();

        let insert_category_description = Query::insert()
            .into_table(CategoryDescription::Table)
            .columns([
                CategoryDescription::CategoryId,
                CategoryDescription::DescriptionId,
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(category_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(description_uuid.clone())),
            ])
            .to_owned();

        let insert_sku_description = Query::insert()
            .into_table(SkuDescription::Table)
            .columns([SkuDescription::SkuId, SkuDescription::DescriptionId])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(sku_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(description_uuid.clone())),
            ])
            .to_owned();

        // Perform DB inserts
        manager.exec_stmt(insert_language).await?;
        manager.exec_stmt(insert_category).await?;
        manager.exec_stmt(insert_product).await?;
        manager.exec_stmt(insert_product_category).await?;
        manager.exec_stmt(insert_sku).await?;
        manager.exec_stmt(insert_sku_product).await?;
        manager.exec_stmt(insert_slug).await?;
        manager.exec_stmt(insert_description).await?;
        manager.exec_stmt(insert_category_description).await?;
        manager.exec_stmt(insert_sku_description).await?;
        manager.exec_stmt(insert_product_description).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Language {
    Table,
    Id,
    Code,
    Name,
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
enum Description {
    Table,
    Id,
    Text,
    LanguageId,
}

#[derive(DeriveIden)]
enum ProductDescription {
    Table,
    ProductId,
    DescriptionId,
}

#[derive(DeriveIden)]
enum SkuDescription {
    Table,
    SkuId,
    DescriptionId,
}

#[derive(DeriveIden)]
enum CategoryDescription {
    Table,
    CategoryId,
    DescriptionId,
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
enum Media {
    Table,
    Id,
    MediaType,
    MediaRole,
    Path,
}

#[derive(DeriveIden)]
enum MediaSet {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum MediaMediaSet {
    Table,
    MediaId,
    MediaSetId,
}

#[derive(DeriveIden)]
enum MediaSetEntity {
    Table,
    Id,
    MediaSetId,
    EntityId,
    EntityType,
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

#[derive(DeriveIden)]
pub enum MediaType {
    #[sea_orm(iden = "media_type")]
    Enum,
    #[sea_orm(iden = "Image")]
    Image,
    #[sea_orm(iden = "Video")]
    Video,
    #[sea_orm(iden = "Document")]
    Document,
}

#[derive(DeriveIden)]
pub enum MediaRole {
    #[sea_orm(iden = "media_role")]
    Enum,
    #[sea_orm(iden = "Gallery")]
    Gallery,
    #[sea_orm(iden = "Description")]
    Description,
    #[sea_orm(iden = "Attachment")]
    Attachment,
}
