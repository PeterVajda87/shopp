use crate::SETTINGS;
use once_cell::sync::Lazy;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

fn init_db_connection() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&SETTINGS.database.connection_string());
    opt.max_connections(100).min_connections(5);

    async_std::task::block_on(async { Database::connect(opt).await.unwrap() })
}

pub static DB: Lazy<DatabaseConnection> = Lazy::new(|| init_db_connection());
