use entities::category;
use openssl::ssl::{SslAcceptor, SslMethod};
use shopp::settings::{Settings, RunMode};
use shopp::Run;
use sea_orm::*;
use std::net::TcpListener;
use sea_orm_migration::prelude::*;
use migrator::Migrator;

mod migrator;
mod entities;

use entities::{prelude::*, *};

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    let run_mode = RunMode::get();
    let settings = Settings::new(&run_mode).expect("Failed to parse settings.");

    let connection: DatabaseConnection = Database::connect(&settings.database.connection_string()).await.expect("Failed to connect to PostgreSQL database");
    Migrator::fresh(&connection).await.expect("Failed to do migrations.");

    let category = category::ActiveModel {
        name: ActiveValue::Set("Cars".to_owned()),
        ..Default::default()
    };

    let _res = Category::insert(category).exec(&connection).await.expect("Failed to insert to DB");

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
