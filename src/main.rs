use shopp::run;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/postgres")
        .await
        .expect("Failed to connect to PostgreSQL database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to do migrations");

    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to a port");
    run(listener, pool)?.await
}
