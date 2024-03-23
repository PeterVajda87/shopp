use uuid::Uuid;

#[allow(dead_code)]
pub struct Category {
    pub category_id: Uuid,
    pub title: String,
    pub parent_category_id: Option<Uuid>,
}
