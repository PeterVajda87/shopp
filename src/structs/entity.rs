use uuid::Uuid;

#[derive(Debug)]
pub struct Entity {
    pub id: Uuid,
    pub entity_type: EntityType,
    pub entity_id: Uuid,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum EntityType {
    Product,
    Category,
    SKU,
}
