use curl::easy::Easy;
use ntex::{web, web::test};
use shopp::{config, run, settings::Settings};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct TestApp {
    pub address: String,
    pub db_pool: Pool<Postgres>,
}
#[ntex::test]
async fn health_check_works() {
    let app = test::init_service(web::App::new().configure(config)).await;
    let req = test::TestRequest::get()
        .uri("/health_check")
        .header("content-type", "text/plain")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[ntex::test]
async fn spawn_server_works() {
    let test_app: TestApp = spawn_server().await;
    let mut easy = Easy::new();
    easy.url(&format!("{}/health_check", &test_app.address))
        .unwrap();
    easy.perform().expect("Failed to perform cUrl request");

    assert_eq!(200, easy.response_code().unwrap());
}

async fn spawn_server() -> TestApp {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    let settings = Settings::new().expect("Failed to parse settings.");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to PostgreSQL database");

    let server = run(listener, db_pool.clone()).expect("Failed to start server.");

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Failed to do migrations");

    let _ = async_std::task::spawn(server);

    TestApp { address, db_pool }
}
