mod routes;
pub mod settings;
mod templates;
use ntex::server::Server;
use ntex::web::{self, types, HttpRequest};
use routes::{home_page, product_page};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use uuid::Uuid;

type DbPool = sqlx::postgres::PgPool;

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

#[web::get("/health_check")]
async fn health_check() -> impl web::Responder {
    web::HttpResponse::Ok()
}

#[web::get("/{slug}")]
async fn route_by_slug(
    req: HttpRequest,
    slug: types::Path<String>,
    pool: types::State<DbPool>,
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
        PageType::Product => {
            return product_page::product_page(req, types::Path::from(slug.item_id), pool).await
        }
        _ => return product_page::product_page(req, types::Path::from(slug.item_id), pool).await,
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(home_page::home_page)
        .service(route_by_slug)
        .service(web::resource("/product/{id}").route(web::get().to(product_page::product_page)));
}

pub fn run(listener: TcpListener, pool: sqlx::PgPool) -> Result<Server, std::io::Error> {
    let server =
        web::HttpServer::new(move || web::App::new().state(pool.clone()).configure(config))
            .listen(listener)?
            .run();

    Ok(server)
}
