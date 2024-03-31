use shopp::{run, settings::Settings};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = Settings::new().expect("Failed to parse settings.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to PostgreSQL database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to do migrations");

    let listener: TcpListener =
        TcpListener::bind("0.0.0.0:8000").expect("Failed to bind to a port");
    run(listener, pool)?.await
}
