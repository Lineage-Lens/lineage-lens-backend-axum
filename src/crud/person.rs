use crate::crud::repository::Repository;
use crate::models::person::Person;
use sqlx::{Error, MySql, Pool};
use std::sync::Arc;

pub struct PersonRepository {
    pub pool: Arc<Pool<MySql>>
}

impl Repository<Person, Error> for PersonRepository {
    async fn find_all(&self) -> Vec<Person> {
        sqlx::query_as!(
            Person,
            "SELECT * FROM person"
        ).fetch_all(self.pool.as_ref()).await.unwrap_or_else(|_| Vec::new())
    }

    async fn save(&self, obj: Person) -> Result<Person, Error> {
        let mut new_person = obj.clone();
        let result = sqlx::query(
            "INSERT INTO person (id, first_name, last_name, birth_date, gender, father_id, mother_id)
                VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(obj.id)
            .bind(obj.first_name)
            .bind(obj.last_name)
            .bind(obj.birth_date)
            .bind(obj.gender.to_string())
            .bind(obj.father_id)
            .bind(obj.mother_id)
            .execute(self.pool.as_ref()).await;
        match result {
            Ok(res) => {
                let id = res.last_insert_id() as i32;
                new_person.id = Option::from(id);
                return Ok(new_person);
            }
            Err(err) => Err(err)
        }
    }
}