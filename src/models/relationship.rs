use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{Error, Row};

use crate::models::util::IntVec;

#[derive(Clone, Serialize, Deserialize, utoipa::ToSchema)]
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

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Relationship {
    pub id: Option<i32>,
    pub relationship_type: RelationshipType,
    pub start_date: NaiveDate,
    #[sqlx(skip)]
    pub people_ids: IntVec,
}

impl <'r>sqlx::FromRow<'r, MySqlRow> for Relationship {
    fn from_row(row: &'r MySqlRow) -> Result<Self, Error> {
        let relationship_type: String = row.try_get("relationship_type")?;
        let relationship_type = RelationshipType::from(relationship_type);

        return Ok(Relationship {
            id: row.try_get("id")?,
            relationship_type,
            start_date: row.try_get("start_date")?,
            people_ids: row.try_get("people_ids").unwrap_or_default(),
        });
    }
}