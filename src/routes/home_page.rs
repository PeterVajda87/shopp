use crate::templates::home_page::HomePage;
use ntex::web::{self, HttpRequest};

#[web::get("/")]
async fn home_page(req: HttpRequest) -> web::HttpResponse {
    println!("Request: {:?}", req);
    web::HttpResponse::Ok().body(HomePage { title: "Homepage" }.to_string())
}
