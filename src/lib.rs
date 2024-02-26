mod templates;
use ntex::server::Server;
use ntex::web::{self, HttpRequest};
use std::net::TcpListener;
use templates::index as IndexTemplate;
use uuid::Uuid;

#[web::get("/health_check")]
async fn health_check() -> impl web::Responder {
    web::HttpResponse::Ok()
}

#[web::get("/")]
async fn index(req: HttpRequest) -> web::HttpResponse {
    println!("Request: {:?}", req);
    web::HttpResponse::Ok().body(IndexTemplate::IndexPage { title: "Homepage" }.to_string())
}

#[web::get("/{slug}")]
async fn route_by_slug(req: HttpRequest, slug: web::types::Path<String>) -> impl web::Responder {
    println!("Request: {:?}", req);
    format!("Hello: {}!", slug)
}

#[web::get("/product/{id}")]
async fn product_page(req: HttpRequest, id: web::types::Path<Uuid>) -> web::HttpResponse {
    println!("Request: {:?}", req);
    web::HttpResponse::Ok().body(
        IndexTemplate::IndexPage {
            title: &format!("Product {} page", id),
        }
        .to_string(),
    )
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(index)
        .service(route_by_slug)
        .service(product_page);
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = web::HttpServer::new(|| web::App::new().configure(config))
        .listen(listener)?
        .run();

    Ok(server)
}
