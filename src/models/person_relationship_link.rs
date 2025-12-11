use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PersonRelationshipLink {
    pub person_id: i32,
    pub relationship_id: i32,
}