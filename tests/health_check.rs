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
