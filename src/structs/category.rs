use uuid::Uuid;

pub struct Category {
    id: Uuid,
    title: String,
    parent_category: Option<Uuid>,
}
