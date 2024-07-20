use super::category::category_page;
use super::product::product_page;
use crate::entities::{self, prelude::*};
use crate::{db::DB, entities::slug};
use ntex::http::StatusCode;
use ntex::web::{self, types::Path, HttpRequest, HttpResponse};
use sea_orm::*;

#[web::get("/{slug}")]
async fn route_by_slug(_req: HttpRequest, slug_text: Path<String>) -> web::HttpResponse {
    let slug_res: Option<entities::slug::Model> = Slug::find()
        .filter(slug::Column::Text.contains(&*slug_text))
        .one(&*DB)
        .await
        .unwrap();

    if let Some(s) = slug_res {
        match s.entity_type {
            entities::sea_orm_active_enums::EntityType::Product => {
                product_page(_req, Path::from(s.entity_id)).await
            }
            entities::sea_orm_active_enums::EntityType::Category => {
                category_page(_req, Path::from(s.entity_id)).await
            }
            _ => HttpResponse::new(StatusCode::NETWORK_AUTHENTICATION_REQUIRED),
        }
    } else {
        HttpResponse::from("Nemam slug_res")
    }
}
