use super::category::category_page;
use super::product::product_page;
use crate::structs::enums::EntityType;
use crate::structs::slug::Slug;
use ntex::http::StatusCode;
use ntex::web::{self, types::Path, HttpRequest, HttpResponse};

#[web::get("/{slug}")]
async fn route_by_slug(_req: HttpRequest, slug_text: Path<String>) -> web::HttpResponse {
    if let Ok(slug) = Slug::get_by_text(&(*slug_text.as_str())).await {
        match slug.target_type {
            EntityType::Category => product_page(_req, Path::from(slug.target_id)).await,
            _ => HttpResponse::new(StatusCode::NETWORK_AUTHENTICATION_REQUIRED),
        }
    } else {
        HttpResponse::from("Nemam slug_res")
    }
}
