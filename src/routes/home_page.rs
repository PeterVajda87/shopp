use crate::templates::index::IndexPage;
use ntex::web::{self, HttpRequest};

#[web::get("/")]
async fn home_page(req: HttpRequest) -> web::HttpResponse {
    println!("Request: {:?}", req);
    web::HttpResponse::Ok().body(IndexPage { title: "Homepage" }.to_string())
}
