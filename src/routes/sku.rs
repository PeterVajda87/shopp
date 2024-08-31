use crate::db::DB;
use ntex::web::{types::Path, HttpRequest, HttpResponse};
use uuid::Uuid;

pub async fn sku_page(_req: HttpRequest, sku_id: Path<Uuid>) -> HttpResponse {
    if let Ok(sku_with_data) = get_sku_with_data(*sku_id).await {
        HttpResponse::Ok().body(
            crate::templates::sku::SkuPage {
                title: &format!("SKU page"),
                sku_data: sku_with_data,
            }
            .to_string(),
        )
    } else {
        HttpResponse::NotFound().body("Nenalezeno")
    }
}

async fn get_sku_with_data(sku_id: Uuid) -> Result<SkuFields, DbErr> {
    let sku_fields = get_sku_fields(sku_id).await;

    match sku_fields {
        Some(sku_fields) => {
            let sku_data = get_sku_data(sku_id).await;
            Ok(SkuData {
                sku_fields,
                media_set: None,
            })
        }
        _ => Err(DbErr::RecordNotFound("Product not found".to_owned())),
    }
}

async fn get_sku_fields(sku_id: Uuid) -> Option<SkuFields> {
    sku::Entity::find()
        .filter(sku::Column::Id.eq(sku_id))
        .select_only()
        .join(
            sea_orm::JoinType::LeftJoin,
            sku::Relation::SkuTranslation.def(),
        )
        .column_as(sku_translation::Column::Name, "name")
        .column_as(sku_translation::Column::Description, "description")
        .into_model::<SkuFields>()
        .one(&*DB)
        .await
        .unwrap()
}
