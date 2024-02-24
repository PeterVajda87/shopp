use ntex::server::Server;
use ntex::web;
use std::net::TcpListener;

#[web::get("/health_check")]
async fn health_check() -> impl web::Responder {
    web::HttpResponse::Ok()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = web::HttpServer::new(|| web::App::new().configure(config))
        .listen(listener)?
        .run();

    Ok(server)
}
