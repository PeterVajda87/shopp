use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "entity_type", rename_all = "lowercase")]
pub enum EntityType {
    Category,
    Product,
    Sku,
}
