use entities::*;
use migrator::Migrator;
use openssl::ssl::{SslAcceptor, SslMethod};
use sea_orm::*;
use sea_orm_migration::prelude::*;
use shopp::{
    settings::{RunMode, Settings},
    Run,
};
use std::net::TcpListener;

mod entities;
mod migrator;

use entities::prelude::*;

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    let run_mode = RunMode::get();
    let settings = Settings::new(&run_mode).expect("Failed to parse settings.");

    let connection: DatabaseConnection = Database::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to PostgreSQL database");
    Migrator::fresh(&connection)
        .await
        .expect("Failed to do migrations.");

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

    let slug = slug::ActiveModel {
        text: ActiveValue::Set("auta".to_owned()),
        item_id: ActiveValue::Set(res_product.last_insert_id),
        item_type: ActiveValue::Set(ItemType::Product),
        ..Default::default()
    };

    let _res_slug = Slug::insert(slug)
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
