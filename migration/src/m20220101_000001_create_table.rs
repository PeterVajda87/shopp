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
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(Language::Code).not_null().string_len(2))
                    .col(ColumnDef::new(Language::Name).not_null().string())
                    .to_owned(),
            )
            .await?;

        // Create mediaset table
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
                    .col(ColumnDef::new(Product::MediaSetId).uuid())
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(ForeignKey::create()
                        .from(Product::Table, Product::MediaSetId)
                        .to(MediaSet::Table, MediaSet::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Create translation table for product
        manager
            .create_table(
                Table::create()
                    .table(ProductTranslation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductTranslation::ProductId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductTranslation::LanguageId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProductTranslation::Name).string().not_null())
                    .col(
                        ColumnDef::new(ProductTranslation::Description)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductTranslation::Table, ProductTranslation::LanguageId)
                            .to(Language::Table, Language::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductTranslation::Table, ProductTranslation::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .table(ProductTranslation::Table)
                            .col(ProductTranslation::ProductId)
                            .col(ProductTranslation::LanguageId),
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
                    .col(
                        ColumnDef::new(Category::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Category::ParentCataegory).uuid())
                    .col(ColumnDef::new(Category::MediaSetId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Category::Table, Category::ParentCataegory)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Category::Table, Category::MediaSetId)
                            .to(MediaSet::Table, MediaSet::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create translation table for category
        manager
            .create_table(
                Table::create()
                    .table(CategoryTranslation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CategoryTranslation::CategoryId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryTranslation::LanguageId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryTranslation::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryTranslation::Description)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CategoryTranslation::Table, CategoryTranslation::LanguageId)
                            .to(Language::Table, Language::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CategoryTranslation::Table, CategoryTranslation::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .table(CategoryTranslation::Table)
                            .col(CategoryTranslation::CategoryId)
                            .col(CategoryTranslation::LanguageId),
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
                    .col(
                        ColumnDef::new(Sku::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Sku::MediaSetId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Sku::Table, Sku::MediaSetId)
                            .to(MediaSet::Table, MediaSet::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create translation table for sku
        manager
            .create_table(
                Table::create()
                    .table(SkuTranslation::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SkuTranslation::SkuId).uuid().not_null())
                    .col(ColumnDef::new(SkuTranslation::LanguageId).uuid().not_null())
                    .col(ColumnDef::new(SkuTranslation::Name).string().not_null())
                    .col(
                        ColumnDef::new(SkuTranslation::Description)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SkuTranslation::Table, SkuTranslation::LanguageId)
                            .to(Language::Table, Language::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SkuTranslation::Table, SkuTranslation::SkuId)
                            .to(Sku::Table, Sku::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .table(SkuTranslation::Table)
                            .col(SkuTranslation::SkuId)
                            .col(SkuTranslation::LanguageId),
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
                    .col(ColumnDef::new(Slug::LanguageId).uuid().not_null())
                    .col(ColumnDef::new(Slug::Text).string().not_null().unique_key())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Slug::Table, Slug::LanguageId)
                            .to(Language::Table, Language::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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

        // Create UUIDs for sample data
        let language_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let language_uuid_2: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let product_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let product_uuid_2: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let category_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let sku_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let sku_uuid_2: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));
        let slug_uuid: Option<Box<Uuid>> = Some(Box::new(Uuid::new_v4()));

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
            .columns([Product::Id])
            .values_panic([SimpleExpr::Value(Value::Uuid(product_uuid.clone()))])
            .values_panic([SimpleExpr::Value(Value::Uuid(product_uuid_2.clone()))])
            .to_owned();

        let insert_category: InsertStatement = Query::insert()
            .into_table(Category::Table)
            .columns([Category::Id])
            .values_panic([SimpleExpr::Value(Value::Uuid(category_uuid.clone()))])
            .to_owned();

        let insert_category_translation = Query::insert()
            .into_table(CategoryTranslation::Table)
            .columns([
                CategoryTranslation::CategoryId,
                CategoryTranslation::LanguageId,
                CategoryTranslation::Name,
                CategoryTranslation::Description,
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(category_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(language_uuid.clone())),
                "Cars".into(),
                "Great things, better than bikes".into(),
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(category_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(language_uuid_2.clone())),
                "Autá".into(),
                "Brm brm vrrrrrrm".into(),
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

        let insert_product_translation = Query::insert()
            .into_table(ProductTranslation::Table)
            .columns([
                ProductTranslation::ProductId,
                ProductTranslation::LanguageId,
                ProductTranslation::Name,
                ProductTranslation::Description,
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(product_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(language_uuid.clone())),
                "Product Skoda 105".into(),
                "Great product for that money!".into(),
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(product_uuid_2.clone())),
                SimpleExpr::Value(Value::Uuid(language_uuid_2.clone())),
                "Produkt Škoda 110".into(),
                "Za hodně peněz málo muziky!".into(),
            ])
            .to_owned();

        let insert_sku: InsertStatement = Query::insert()
            .into_table(Sku::Table)
            .columns([Sku::Id])
            .values_panic([SimpleExpr::Value(Value::Uuid(sku_uuid.clone()))])
            .values_panic([SimpleExpr::Value(Value::Uuid(sku_uuid_2.clone()))])
            .to_owned();

        let insert_sku_translation = Query::insert()
            .into_table(SkuTranslation::Table)
            .columns([
                SkuTranslation::SkuId,
                SkuTranslation::LanguageId,
                SkuTranslation::Name,
                SkuTranslation::Description,
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(sku_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(language_uuid.clone())),
                "Family Sedan".into(),
                "Now we a lot longer ass!".into(),
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(sku_uuid_2.clone())),
                SimpleExpr::Value(Value::Uuid(language_uuid_2.clone())),
                "Sport kupé".into(),
                "Devadesát!".into(),
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
            .columns([
                Slug::Id,
                Slug::Text,
                Slug::EntityType,
                Slug::EntityId,
                Slug::LanguageId,
            ])
            .values_panic([
                SimpleExpr::Value(Value::Uuid(slug_uuid)),
                "Auta".into(),
                Expr::val("Product").as_enum(Alias::new("entity_type")),
                SimpleExpr::Value(Value::Uuid(product_uuid.clone())),
                SimpleExpr::Value(Value::Uuid(language_uuid.clone())),
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
        manager.exec_stmt(insert_category_translation).await?;
        manager.exec_stmt(insert_product_translation).await?;
        manager.exec_stmt(insert_sku_translation).await?;

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
    CreatedAt,
    MediaSetId
}

#[derive(DeriveIden)]
enum ProductTranslation {
    Table,
    ProductId,
    LanguageId,
    Name,
    Description,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    ParentCataegory,
    CreatedAt,
    MediaSetId
}

#[derive(DeriveIden)]
enum CategoryTranslation {
    Table,
    CategoryId,
    LanguageId,
    Name,
    Description,
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
    CreatedAt,
    MediaSetId
}

#[derive(DeriveIden)]
enum SkuTranslation {
    Table,
    SkuId,
    LanguageId,
    Name,
    Description,
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
    LanguageId,
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
    MediaSetId
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
