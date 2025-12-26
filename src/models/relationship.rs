use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Dating,
    Engaged,
    Married,
}

impl RelationshipType {
    pub fn to_string(&self) -> String {
        match self {
            RelationshipType::Dating => "DATING",
            RelationshipType::Engaged => "ENGAGED",
            RelationshipType::Married => "MARRIED"
        }.to_string()
    }
}

impl From<String> for RelationshipType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "DATING" => RelationshipType::Dating,
            "ENGAGED" => RelationshipType::Engaged,
            "MARRIED" => RelationshipType::Married,
            _ => RelationshipType::Dating
        }
    }
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Relationship {
    pub id: Option<i32>,
    pub relationship_type: RelationshipType,
    pub start_date: NaiveDate,
}