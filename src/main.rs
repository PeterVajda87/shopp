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

    let gallery = gallery::ActiveModel {
        ..Default::default()
    };

    let _res_gal = Gallery::insert(gallery)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB."); 


    let image = media::ActiveModel {
        r#type: ActiveValue::Set(entities::sea_orm_active_enums::MediaType::Image),
        path: ActiveValue::Set("/static/product/prd.webp".to_string()),
        gallery_id: ActiveValue::Set(Some(_res_gal.last_insert_id)),
        ..Default::default()
    };

    let _res_image = Media::insert(image)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB.");
    
    let category = category::ActiveModel {
        name: ActiveValue::Set("cars".to_owned()),
        ..Default::default()
    };

    let _res_category = Category::insert(category)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB");

    let product = product::ActiveModel {
        name: ActiveValue::Set("Skoda 105".to_owned()),
        category_id: ActiveValue::Set(_res_category.last_insert_id),
        gallery_id: ActiveValue::Set(Some(_res_gal.last_insert_id)),
        ..Default::default()
    };

    let res_product: InsertResult<product::ActiveModel> = Product::insert(product)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB");

    let lang = language::ActiveModel {
        code: ActiveValue::Set("CZ".to_string()),
        name: ActiveValue::Set("Čeština".to_string()),
        ..Default::default()
    };

    let res_lang = Language::insert(lang)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB");

    let slug = slug::ActiveModel {
        text: ActiveValue::Set("auta".to_owned()),
        entity_id: ActiveValue::Set(res_product.last_insert_id),
        entity_type: ActiveValue::Set(entities::sea_orm_active_enums::EntityType::Product),
        language_code: ActiveValue::Set(res_lang.last_insert_id),
        ..Default::default()
    };

    let _res_slug = Slug::insert(slug)
        .exec(&connection)
        .await
        .expect("Failed to insert to DB.");

    let sku = sku::ActiveModel {
        name: ActiveValue::Set("modra skoda 105".to_string()),
        product_id: ActiveValue::Set(res_product.last_insert_id),
        ..Default::default()
    };

    let _res_sku = Sku::insert(sku)
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
