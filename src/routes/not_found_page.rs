use ntex::web::{HttpRequest, HttpResponse};

pub async fn not_found_page(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().into()
}
