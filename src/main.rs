use openssl::ssl::{SslAcceptor, SslMethod};
use sea_orm::*;
use shopp::{
    settings::{RunMode, Settings},
    Run,
};
use std::net::TcpListener;

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    let run_mode = RunMode::get();
    let settings = Settings::new(&run_mode).expect("Failed to parse settings.");

    let connection: DatabaseConnection = Database::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to PostgreSQL database");

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
