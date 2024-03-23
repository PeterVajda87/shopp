pub mod models;
mod routes;
pub mod settings;
mod templates;
use ntex::{
    server::Server,
    web::{self, types::Path, Error},
};
use ntex_files as fs;
use routes::{home_page::*, product_page::*, slug::*};
use std::net::TcpListener;

pub type DbPool = sqlx::postgres::PgPool;

#[web::get("/health_check")]
async fn health_check() -> impl web::Responder {
    web::HttpResponse::Ok()
}

#[web::get("/static/{file_path}")]
async fn static_file(file_path: Path<String>) -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open(format!("static/{file_path}"))?)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(static_file)
        .service(home_page)
        .service(route_by_slug)
        .service(web::resource("/product/{id}").route(web::get().to(product_page)));
}

pub fn run(listener: TcpListener, pool: sqlx::PgPool) -> Result<Server, std::io::Error> {
    let server =
        web::HttpServer::new(move || web::App::new().state(pool.clone()).configure(config))
            .listen(listener)?
            .run();

    Ok(server)
}
