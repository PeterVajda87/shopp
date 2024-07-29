use crate::db::DB;
use crate::entities::*;
use crate::structs::product::{ProductFields, ProductData};
use crate::structs::sku::{SkuFields, SkuData};
use ntex::web::{types::Path, HttpRequest, HttpResponse};
use sea_orm::*;
use uuid::Uuid;

pub async fn product_page(_req: HttpRequest, product_id: Path<Uuid>) -> HttpResponse {
    if let Ok(product_with_data) = get_product_with_data(*product_id).await {
        HttpResponse::Ok().body(
            crate::templates::product::ProductPage {
                title: &format!("Product page"),
                product_data: product_with_data,
            }
            .to_string(),
        )
    } else {
        HttpResponse::NotFound().body("Nenalezeno")
    }
}

async fn get_product_with_data(product_id: Uuid) -> Result<ProductFields, DbErr> {
    let product_fields = get_product_fields(product_id).await;

    match product_fields {
        Some(product_fields) => {
            let sku_data = get_sku_data(product_id).await;
            Ok(ProductWithSku {
                product_data,
                sku_data,
            })
        }
        _ => Err(DbErr::RecordNotFound("Product not found".to_owned())),
    }
}

async fn get_product_fields(product_id: Uuid) -> Option<ProductData> {
    product::Entity::find()
        .filter(product::Column::Id.eq(product_id))
        .select_only()
        .join(
            sea_orm::JoinType::LeftJoin,
            product::Relation::ProductDescription.def(),
        )
        .join(
            sea_orm::JoinType::LeftJoin,
            product_description::Relation::Description.def(),
        )
        .column_as(product::Column::Name, "name")
        .column_as(description::Column::Text, "description")
        .into_model::<ProductData>()
        .one(&*DB)
        .await
        .unwrap()
}


// async fn get_sku_data(product_id: Uuid) -> Vec<SkuData> {
//     sku::Entity::find()
//         .select_only()
//         .join(
//             sea_orm::JoinType::LeftJoin,
//             sku::Relation::SkuDescription.def(),
//         )
//         .join(
//             sea_orm::JoinType::LeftJoin,
//             sku_description::Relation::Description.def(),
//         )
//         .join(sea_orm::JoinType::LeftJoin, sku::Relation::SkuProduct.def())
//         .join(
//             sea_orm::JoinType::LeftJoin,
//             sku_product::Relation::Product.def(),
//         )
//         .filter(product::Column::Id.eq(product_id))
//         .column_as(sku::Column::Name, "name")
//         .column_as(description::Column::Text, "description")
//         .into_model::<SkuData>()
//         .all(&*DB)
//         .await
//         .unwrap()
// }
