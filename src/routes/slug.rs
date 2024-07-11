use super::product::product_page;
use crate::db::DB;
use crate::structs::{Entity, EntityType};
use ntex::web::{self, types::Path, HttpRequest, HttpResponse};

#[web::get("/{slug}")]
async fn route_by_slug(_req: HttpRequest, slug: Path<String>) -> HttpResponse {
    let entity = sqlx::query_as!(
        Entity,
        r#"SELECT id, entity_id, entity_type as "entity_type: EntityType" FROM entity WHERE id = (SELECT entity_id FROM slug WHERE text = $1)"#,
        *slug
    )
    .fetch_one(&*DB)
    .await;

    println!("{:?}", entity);

    if let Ok(entity_object) = entity {
        match entity_object.entity_type {
            EntityType::Product => product_page(_req, Path::from(entity_object.entity_id)).await,
            _ => HttpResponse::Ok().finish(),
        }
    } else {
        HttpResponse::Ok().finish()
    }
}
