use uuid::Uuid;

pub struct Slug {
    pub text: String,
    pub entity_id: Uuid,
    pub language_id: Uuid,
}
