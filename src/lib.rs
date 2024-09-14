pub mod db;
pub mod middleware;
pub mod routes;
pub mod settings;
pub mod structs;
pub mod templates;

use language::Language;
use middleware::auth::JwtAuth;
use ntex::web::{
    get, post, resource,
    types::{Json, Path},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, ServiceConfig,
};
use ntex_files as fs;
use once_cell::sync::Lazy;
use routes::*;
use settings::{RunMode, Settings};
use structs::{category::Category, *};
use traits::{FromRequest, Storable};

pub static RUN_MODE: Lazy<RunMode> = Lazy::new(|| RunMode::get());
pub static SETTINGS: Lazy<Settings> =
    Lazy::new(|| Settings::new(&RUN_MODE).expect("Failed to parse settings."));

pub trait Run {
    fn run(self, settings: &Settings) -> Result<ntex::server::Server, std::io::Error>;
}

impl Run for std::net::TcpListener {
    fn run(self, _settings: &Settings) -> Result<ntex::server::Server, std::io::Error> {
        let server = HttpServer::new(move || App::new().wrap(JwtAuth).configure(config))
            .listen(self)?
            .run();
        Ok(server)
    }
}

impl Run for openssl::ssl::SslAcceptorBuilder {
    fn run(mut self, settings: &Settings) -> Result<ntex::server::Server, std::io::Error> {
        if let Some(ssl) = &settings.ssl {
            self.set_private_key_file(&ssl.private_key_file, openssl::ssl::SslFiletype::PEM)
                .expect("Error loading private key file.");
            self.set_certificate_chain_file(&ssl.certification_chain_file)
                .expect("Error loading certification chain file.");
            let server = HttpServer::new(move || App::new().wrap(JwtAuth).configure(config))
                .bind_openssl(("0.0.0.0", settings.application_port), self)?
                .run();

            Ok(server)
        } else {
            panic!("Missing SSL files paths")
        }
    }
}

#[get("/health_check")]
async fn health_check(_req: HttpRequest) -> impl Responder {
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

#[post("/create/create_language")]
async fn create_language(req: Json<Language>) -> HttpResponse {
    match Language::create_from_request(req).await {
        Ok(new_lang) => match new_lang.insert().await {
            Ok(_res) => HttpResponse::Ok().finish(),
            Err(err) => {
                println!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(), // Handle error case
    }
}

#[post("/create/create_category")]
async fn create_category(req: Json<Category>) -> HttpResponse {
    match Category::create_from_request(req).await {
        Ok(new_cat) => match new_cat.insert().await {
            Ok(_res) => HttpResponse::Ok().finish(),
            Err(err) => {
                println!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(), // Handle error case
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(health_check)
        .service(catalog_file)
        .service(route_by_slug)
        .service(static_file)
        .service(create_language)
        .service(create_category)
        .service(resource("/product/{id}").route(get().to(product_page)))
        .service(resource("/category/{id}").route(get().to(category_page)));
}
