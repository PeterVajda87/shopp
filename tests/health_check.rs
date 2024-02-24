use curl::easy::Easy;
use ntex::{web, web::test};
use shopp::config;

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
    let address = spawn_server();
    spawn_server();

    let mut easy = Easy::new();
    easy.url(&format!("{}/health_check", &address)).unwrap();
    easy.perform().expect("Failed to perform cUrl request");

    assert_eq!(200, easy.response_code().unwrap());
}

fn spawn_server() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = shopp::run(listener).expect("Failed to start server.");
    let _ = async_std::task::spawn(server);

    format!("http://127.0.0.1:{port}")
}
