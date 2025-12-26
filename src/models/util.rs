use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlTypeInfo, MySqlValueRef};
use sqlx::types::Json;
use sqlx::{Decode, MySql, Type};
use std::ops::{Deref, DerefMut};

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