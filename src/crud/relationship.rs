use crate::crud::repository::Repository;
use sqlx::{Error, MySql, Pool};
use std::sync::Arc;
use crate::models::relationship::Relationship;

pub struct RelationshipRepository {
    pub pool: Arc<Pool<MySql>>
}

impl Repository<Relationship, Error> for RelationshipRepository {
    async fn find_all(&self) -> Vec<Relationship> {
        sqlx::query_as(
            "SELECT * FROM relationship"
        ).fetch_all(self.pool.as_ref()).await.unwrap_or_else(|_| Vec::new())
    }

    async fn save(&self, obj: Relationship) -> Result<Relationship, Error> {
        let mut new_relationship = obj.clone();
        let result = sqlx::query(
            "INSERT INTO relationship (id, relationship_type, start_date)
                VALUES (?, ?, ?)")
            .bind(obj.id)
            .bind(obj.relationship_type.to_string())
            .bind(obj.start_date)
            .execute(self.pool.as_ref()).await;
        match result {
            Ok(res) => {
                let id = res.last_insert_id() as i32;
                new_relationship.id = Option::from(id);
                return Ok(new_relationship);
            }
            Err(err) => Err(err)
        }
    }
}