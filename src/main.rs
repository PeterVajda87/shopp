use openssl::ssl::{SslAcceptor, SslMethod};
use shopp::settings::RunMode;
use shopp::{settings::Settings, Run};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    let run_mode = RunMode::get();
    let settings = Settings::new(&run_mode).expect("Failed to parse settings.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to PostgreSQL database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to do migrations");

    match run_mode {
        RunMode::Production => {
            let ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            ssl_builder.run(pool, settings)?.await
        }
        RunMode::Development => {
            let tcp_listener = TcpListener::bind(("0.0.0.0", settings.application_port)).unwrap();
            tcp_listener.run(pool, settings)?.await
        }
    }
}
