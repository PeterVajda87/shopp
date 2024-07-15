use super::category::category_page;
use super::product::product_page;
use crate::db::DB;
use ntex::web::{self, types::Path, HttpRequest, HttpResponse};

#[web::get("/{slug}")]
async fn route_by_slug(_req: HttpRequest, _slug: Path<String>) -> HttpResponse {
    todo!()
}
