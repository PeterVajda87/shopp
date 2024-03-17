use crate::routes::product_page::*;
use crate::DbPool;
use ntex::web::{
    self,
    types::{Path, State},
    HttpRequest,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "page_type", rename_all = "lowercase")]
enum PageType {
    Product,
    Category,
    CMS,
    Checkout,
    Cart,
}

#[derive(Debug)]
struct Slug {
    slug: String,
    page_type: PageType,
    item_id: Uuid,
}

struct Category {
    id: Uuid,
    category_name: String,
}

#[web::get("/{slug}")]
async fn route_by_slug(
    req: HttpRequest,
    slug: Path<String>,
    pool: State<DbPool>,
) -> impl web::Responder {
    println!("Slug: {:?}", slug);
    let slug = sqlx::query_as!(
        Slug,
        r#"SELECT slug, page_type as "page_type: PageType", item_id FROM slug WHERE slug = $1"#,
        slug.as_str()
    )
    .fetch_one(&*pool)
    .await
    .expect("Non existing slug, TODO");

    match slug.page_type {
        PageType::Product => return product_page(req, Path::from(slug.item_id), pool).await,
        _ => return product_page(req, Path::from(slug.item_id), pool).await,
    }
}
