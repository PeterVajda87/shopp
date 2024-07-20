use crate::entities::prelude::Category;
use crate::structs::product::ProductData;
use ntex::web::{types::Path, HttpRequest, HttpResponse};
use uuid::Uuid;

pub struct CategoryData {
    pub category: Category,
    pub products: Vec<ProductData>,
}

pub async fn category_page(_req: HttpRequest, _id: Path<Uuid>) -> HttpResponse {
    todo!();
}
