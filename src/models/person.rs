use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Gender {
    MALE,
    FEMALE
}

impl Gender {
    pub fn to_string(&self) -> String {
        match self {
            Gender::MALE => "MALE",
            Gender::FEMALE => "FEMALE"
        }.to_string()
    }
}

impl From<String> for Gender {
    fn from(value: String) -> Self {
        match value.as_str() {
            "MALE" => Gender::MALE,
            "FEMALE" => Gender::FEMALE,
            _ => Gender::MALE
        }
    }
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Person {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub gender: Gender,
    pub father_id: Option<i32>,
    pub mother_id: Option<i32>,
}