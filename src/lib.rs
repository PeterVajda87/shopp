use ntex::server::Server;
use ntex::web;

#[web::get("/health_check")]
async fn health_check() -> impl web::Responder {
    web::HttpResponse::Ok()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = web::HttpServer::new(|| web::App::new().configure(config))
        .bind(("127.0.0.1", 8000))?
        .run();

    Ok(server)
}
