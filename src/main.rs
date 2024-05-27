use entities::*;
use openssl::ssl::{SslAcceptor, SslMethod};
use sea_orm::*;
use shopp::{
    settings::{RunMode, Settings},
    Run,
};
use std::net::TcpListener;

mod entities;
use entities::prelude::*;

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    let run_mode = RunMode::get();
    let settings = Settings::new(&run_mode).expect("Failed to parse settings.");

    let connection: DatabaseConnection = Database::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to PostgreSQL database");

    let category = category::ActiveModel {
        name: ActiveValue::Set("cars".to_owned()),
        ..Default::default()
    };

    let res_category = Category::insert(category)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB");

    let product = product::ActiveModel {
        name: ActiveValue::Set("Skoda 105".to_owned()),
        ..Default::default()
    };

    let res_product: InsertResult<product::ActiveModel> = Product::insert(product)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB");

    let p2c = product_to_category::ActiveModel {
        product_id: ActiveValue::Set(res_product.last_insert_id),
        category_id: ActiveValue::Set(res_category.last_insert_id),
        ..Default::default()
    };

    let _res_p2c = ProductToCategory::insert(p2c)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB");

    let lang = language::ActiveModel {
        language_code: ActiveValue::Set("CZ".to_string()),
        name: ActiveValue::Set("Čeština".to_string()),
        ..Default::default()
    };

    let res_lang = Language::insert(lang)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB");

    let slug = slug::ActiveModel {
        text: ActiveValue::Set("auta".to_owned()),
        item_id: ActiveValue::Set(res_product.last_insert_id),
        item_type: ActiveValue::Set(ItemType::Product),
        language_id: ActiveValue::Set(res_lang.last_insert_id),
        ..Default::default()
    };

    let _res_slug = Slug::insert(slug)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB.");

    let image = media::ActiveModel {
        media_type: ActiveValue::Set(MediaType::Image),
        path: ActiveValue::Set("/static/product/prd.webp".to_string()),
        ..Default::default()
    };

    let res_image = Media::insert(image)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB.");

    let image_to_product = media_to_item::ActiveModel {
        media_id: ActiveValue::Set(res_image.last_insert_id),
        item_id: ActiveValue::Set(res_product.last_insert_id),
        item_type: ActiveValue::Set(ItemType::Product),
        order: ActiveValue::set(0),
        ..Default::default()
    };

    let _res_i2p = MediaToItem::insert(image_to_product)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB.");

    let sku = sku::ActiveModel {
        name: ActiveValue::Set("modra skoda 105".to_string()),
        ..Default::default()
    };

    let res_sku = Sku::insert(sku)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB.");

    let s2p = sku_to_product::ActiveModel {
        product_id: ActiveValue::Set(res_product.last_insert_id),
        sku_id: ActiveValue::Set(res_sku.last_insert_id),
        ..Default::default()
    };

    let _res_s2p = SkuToProduct::insert(s2p)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB.");

    match run_mode {
        RunMode::Production => {
            let ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            ssl_builder.run(connection, settings)?.await
        }
        RunMode::Development => {
            let tcp_listener = TcpListener::bind(("0.0.0.0", settings.application_port)).unwrap();
            tcp_listener.run(connection, settings)?.await
        }
    }
}
