use crate::templates::product_page::ProductPage;
use ntex::web::{self, HttpRequest};
use uuid::Uuid;

struct Product {
    id: Uuid,
    title: String,
}

#[web::get("/product/{id}")]
async fn product_page(req: HttpRequest, id: web::types::Path<Uuid>) -> web::HttpResponse {
    println!("Request: {:?}", req);
    web::HttpResponse::Ok().body(
        ProductPage {
            title: &format!("Product {} page", id),
        }
        .to_string(),
    )
}
