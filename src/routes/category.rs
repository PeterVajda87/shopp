use ntex::web::{types::Path, HttpRequest, HttpResponse};
use uuid::Uuid;

pub async fn category_page(_req: HttpRequest, _id: Path<Uuid>) -> HttpResponse {
    todo!();
}
