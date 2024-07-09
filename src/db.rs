use once_cell::sync::Lazy;
use sea_orm::{DatabaseConnection, Database};
use crate::settings;

fn init_db_connection() -> DatabaseConnection {
    let run_mode = settings::RunMode::get();
    let settings = settings::Settings::new(&run_mode).expect("Failed to parse settings."); 

    async_std::task::block_on(async {
        Database::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to PostgreSQL database")
    })
}

pub static DB: Lazy<DatabaseConnection> = Lazy::new(|| init_db_connection());
