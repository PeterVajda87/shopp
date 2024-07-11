pub mod db;
pub mod routes;
pub mod settings;
pub mod structs;
pub mod templates;

use ntex::web::{
    get, resource, types::Path, App, Error, HttpResponse, HttpServer, Responder, ServiceConfig,
};
use ntex_files as fs;
use once_cell::sync::Lazy;
use openssl::ssl::SslFiletype;
use routes::{product::product_page, slug::route_by_slug};
use settings::{RunMode, Settings};
use uuid::Uuid;

pub static DUMMY_UUID: Lazy<Uuid> = Lazy::new(|| {
    Uuid::parse_str("11111111-1111-4111-8111-111111111111").expect("Failed to parse UUID")
});
pub static RUN_MODE: Lazy<RunMode> = Lazy::new(|| RunMode::get());
pub static SETTINGS: Lazy<Settings> =
    Lazy::new(|| Settings::new(&RUN_MODE).expect("Failed to parse settings."));

pub trait Run {
    fn run(self, settings: &Settings) -> Result<ntex::server::Server, std::io::Error>;
}

impl Run for std::net::TcpListener {
    fn run(self, _settings: &Settings) -> Result<ntex::server::Server, std::io::Error> {
        let server = HttpServer::new(move || App::new().configure(config))
            .listen(self)?
            .run();

        Ok(server)
    }
}

impl Run for openssl::ssl::SslAcceptorBuilder {
    fn run(mut self, settings: &Settings) -> Result<ntex::server::Server, std::io::Error> {
        if let Some(ssl) = &settings.ssl {
            self.set_private_key_file(&ssl.private_key_file, SslFiletype::PEM)
                .expect("Error loading private key file.");
            self.set_certificate_chain_file(&ssl.certification_chain_file)
                .expect("Error loading certification chain file.");
            self.set_ca_file(&ssl.ca_file)
                .expect("CA bundle not found or could not be loaded.");
            let server = HttpServer::new(move || App::new().configure(config))
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
        .service(route_by_slug)
        .service(static_file)
        .service(resource("/product/{id}").route(get().to(product_page)));
}
