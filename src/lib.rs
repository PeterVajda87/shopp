pub mod models;
mod routes;
pub mod settings;
mod templates;
use ntex::{
    server::Server,
    web::{
        get, resource, types::Path, App, Error, HttpResponse, HttpServer, Responder, ServiceConfig,
    },
};
use ntex_files as fs;
use routes::{home_page::*, product_page::*, slug::*};
use std::net::TcpListener;

pub type DbPool = sqlx::postgres::PgPool;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/static/{file_path}")]
async fn static_file(file_path: Path<String>) -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open(format!("static/{file_path}"))?)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(health_check)
        .service(static_file)
        .service(home_page)
        .service(route_by_slug)
        .service(resource("/product/{id}").route(get().to(product_page)));
}

pub fn run(listener: TcpListener, pool: sqlx::PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || App::new().state(pool.clone()).configure(config))
        .listen(listener)?
        .run();

    Ok(server)
}
