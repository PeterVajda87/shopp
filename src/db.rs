use crate::SETTINGS;
use once_cell::sync::Lazy;
use sqlx::PgPool;

fn init_db_connection() -> PgPool {
    async_std::task::block_on(async {
        PgPool::connect(&SETTINGS.database.connection_string())
            .await
            .unwrap()
    })
}

pub static DB_POOL: Lazy<PgPool> = Lazy::new(|| init_db_connection());
