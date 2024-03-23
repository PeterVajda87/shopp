use markup::Render;
use uuid::Uuid;

#[allow(dead_code)]
pub struct Product {
    pub product_id: Uuid,
    pub title: String,
    pub category_id: Option<Uuid>,
}

impl Render for Product {
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.title.render(writer)
    }
}
