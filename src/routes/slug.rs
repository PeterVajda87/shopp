use crate::entities::{self, prelude::*, slug};
use crate::routes::product::product_page;
use ntex::web::{
    self,
    types::Path,
    HttpRequest, HttpResponse,
};
use sea_orm::*;
use crate::db::DB;

#[web::get("/{slug}")]
async fn route_by_slug(
    _req: HttpRequest,
    slug: Path<String>) -> HttpResponse {
    let slug_opt: Option<slug::Model> = Slug::find()
        .filter(slug::Column::Text.eq(slug.as_str()))
        .one(&*DB)
        .await
        .expect("Failed to execute search query");

    if let Some(slug) = slug_opt {
        println!("{:?}", &slug.entity_type);
        match slug.entity_type {
            entities::sea_orm_active_enums::EntityType::Product => {
                product_page(_req, Path::from(slug.entity_id)).await
            }
            entities::sea_orm_active_enums::EntityType::Category => HttpResponse::Ok().finish(),
            _ => HttpResponse::Ok().finish(),
        }
    } else {
        HttpResponse::Ok().finish()
    }
}
