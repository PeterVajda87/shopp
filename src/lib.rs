mod routes;
mod templates;
use ntex::server::Server;
use ntex::web::{self, HttpRequest};
use std::net::TcpListener;
use uuid::Uuid;

enum PageType {
    Product,
    Category,
    CMS,
    Checkout,
}

struct Slug {
    id: Uuid,
    text: String,
    page_type: PageType,
}

#[web::get("/health_check")]
async fn health_check() -> impl web::Responder {
    web::HttpResponse::Ok()
}

#[web::get("/{slug}")]
async fn route_by_slug(req: HttpRequest, slug: web::types::Path<String>) -> impl web::Responder {
    println!("Request: {:?}", req);
    format!("Hello: {}!", slug)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(routes::home_page::home_page)
        .service(route_by_slug)
        .service(routes::product_page::product_page);
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = web::HttpServer::new(|| web::App::new().configure(config))
        .listen(listener)?
        .run();

    Ok(server)
}
