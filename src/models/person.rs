use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlRow, MySqlTypeInfo, MySqlValueRef};
use sqlx::types::Json;
use sqlx::{Decode, Error, MySql, Row, Type};
use std::ops::{Deref, DerefMut};

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
    #[sqlx(skip)]
    pub children_ids: IntVec,
}

impl <'r>sqlx::FromRow<'r, MySqlRow> for Person {
    fn from_row(row: &'r MySqlRow) -> Result<Self, Error> {
        let tt: String = row.try_get("gender")?;
        let gender = Gender::from(tt);

        return Ok(Person {
            id: row.try_get("id")?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            birth_date: row.try_get("birth_date")?,
            gender,
            father_id: row.try_get("father_id")?,
            mother_id: row.try_get("mother_id")?,
            children_ids: row.try_get("children_ids").unwrap_or_default(),
        });
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct IntVec(pub Vec<i32>);

impl Deref for IntVec {
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IntVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'r> Decode<'r, sqlx::MySql> for IntVec {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let json: Json<Vec<i32>> = Json::decode(value)?;
        Ok(IntVec(json.0))
    }
}

impl Type<MySql> for IntVec {
    fn type_info() -> <MySql as sqlx::Database>::TypeInfo {
        <Json<Vec<i32>>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <Json<Vec<i32>> as Type<MySql>>::compatible(ty)
    }
}