pub mod models;
mod routes;
pub mod settings;
mod templates;
use ntex::web::{
    get, resource, types::Path, App, Error, HttpResponse, HttpServer, Responder, ServiceConfig,
};
use ntex_files as fs;
use openssl::ssl::SslFiletype;
use routes::{home_page::*, not_found_page::*, product_page::*, slug::*};
use settings::Settings;

pub type DbPool = sqlx::postgres::PgPool;

pub trait Run {
    fn run(
        self,
        pool: sqlx::PgPool,
        settings: Settings,
    ) -> Result<ntex::server::Server, std::io::Error>;
}

impl Run for std::net::TcpListener {
    fn run(
        self,
        pool: sqlx::PgPool,
        _settings: Settings,
    ) -> Result<ntex::server::Server, std::io::Error> {
        let server = HttpServer::new(move || {
            App::new()
                .default_service(ntex::web::to(|req| async { not_found_page(req).await }))
                .state(pool.clone())
                .configure(config)
        })
        .listen(self)?
        .run();

        Ok(server)
    }
}

impl Run for openssl::ssl::SslAcceptorBuilder {
    fn run(
        mut self,
        pool: sqlx::PgPool,
        settings: Settings,
    ) -> Result<ntex::server::Server, std::io::Error> {
        if let Some(ssl) = settings.ssl {
            self.set_private_key_file(ssl.private_key_file, SslFiletype::PEM)
                .expect("Error loading private key file.");
            self.set_certificate_chain_file(ssl.certification_chain_file)
                .expect("Error loading certification chain file.");
            self.set_ca_file(ssl.ca_file)
                .expect("CA bundle not found or could not be loaded.");
            let server = HttpServer::new(move || {
                App::new()
                    .default_service(ntex::web::to(|req| async { not_found_page(req).await }))
                    .state(pool.clone())
                    .configure(config)
            })
            .bind_openssl(("0.0.0.0", settings.application_port), self)?
            .run();

            Ok(server)
        } else {
            panic!("Missing SSL files paths")
        }
    }
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/static/{directory}/{file_path}")]
async fn catalog_file(path: Path<(String, String)>) -> Result<fs::NamedFile, Error> {
    let (directory, file_path) = path.into_inner();
    Ok(fs::NamedFile::open(format!(
        "static/{directory}/{file_path}"
    ))?)
}

#[get("/static/{file_path}")]
async fn static_file(file_path: Path<String>) -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open(format!("static/{file_path}"))?)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(health_check)
        .service(catalog_file)
        .service(static_file)
        .service(home_page)
        .service(route_by_slug)
        .service(resource("/product/{id}").route(get().to(product_page)));
}
