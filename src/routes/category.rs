use crate::entities::{category, prelude::Category};
use ntex::web::{
    types::{Path, State},
    HttpRequest, HttpResponse,
};
use sea_orm::DatabaseConnection;
use sea_orm::*;
use uuid::Uuid;

pub async fn category_page(
    _req: HttpRequest,
    id: Path<Uuid>,
    conn: State<DatabaseConnection>,
) -> HttpResponse {
    let category_opt: Option<category::Model> = Category::find()
        .filter(category::Column::Id.eq(*id))
        .one(&*conn)
        .await
        .expect("Failed to execute search query");

    if let Some(category) = category_opt {
        HttpResponse::Ok().body(
            crate::templates::category::CategoryPage {
                title: &format!("Category {} page", category.name),
                category,
            }
            .to_string(),
        )
    } else {
        HttpResponse::from("kategoriu som nenasiel".to_string())
    }
}
