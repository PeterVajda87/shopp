use crate::db::DB;
use ntex::web::{types::Path, HttpRequest, HttpResponse};
use uuid::Uuid;

pub async fn product_page(_req: HttpRequest, _product_id: Path<Uuid>) -> HttpResponse {
    // if let Some(product) = product_data {
    //     HttpResponse::Ok().body(
    //         crate::templates::product::ProductPage {
    //             title: &format!("Product page"),
    //             product_data: product,
    //         }
    //         .to_string(),
    //     )
    // } else {
    //     HttpResponse::Ok().body("ABC".to_string())
    // }
    todo!()
}

fn product_data() -> Option<String> {
    todo!()
}
