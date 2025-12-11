use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    DATING,
    ENGAGED,
    MARRIED,
}

impl RelationshipType {
    pub fn to_string(&self) -> String {
        match self {
            RelationshipType::DATING => "DATING",
            RelationshipType::ENGAGED => "ENGAGED",
            RelationshipType::MARRIED => "MARRIED"
        }.to_string()
    }
}

impl From<String> for RelationshipType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "DATING" => RelationshipType::DATING,
            "ENGAGED" => RelationshipType::ENGAGED,
            "MARRIED" => RelationshipType::MARRIED,
            _ => RelationshipType::DATING
        }
    }
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Relationship {
    pub id: Option<i32>,
    pub relationship_type: RelationshipType,
    pub start_date: NaiveDate,
}