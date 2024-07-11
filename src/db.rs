use once_cell::sync::Lazy;
use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::SETTINGS;

fn init_db_connection() -> PgPool {
    async_std::task::block_on(async {
        PgPoolOptions::new()
        .max_connections(5)
        .connect(&SETTINGS.database.connection_string())
        .await
        .expect("Failed to connect to PostgreSQL database")
    })
}

pub static DB: Lazy<PgPool> = Lazy::new(|| init_db_connection());
