use uuid::Uuid;

#[derive(Debug)]
#[derive(sqlx::Type)]
pub struct Description {
    pub id: Uuid,
    pub text: String,
    pub entity_id: Uuid,
    pub language_id: Uuid,
}
