use crate::crud::repository::Repository;
use crate::models::person_relationship_link::PersonRelationshipLink;
use sqlx::{Error, MySql, Pool};
use std::sync::Arc;

pub struct PersonRelationshipLinkRepository {
    pub pool: Arc<Pool<MySql>>
}

impl PersonRelationshipLinkRepository {
    pub async fn save_all(&self, objs: Vec<PersonRelationshipLink>) -> Result<(), Error> {
        for obj in objs {
            match self.save(obj).await {
                Ok(_) => (),
                Err(e) => return Err(e)
            };
        }
        return Ok(());
    }
}

impl Repository<PersonRelationshipLink, Error> for PersonRelationshipLinkRepository {
    async fn find_all(&self) -> Vec<PersonRelationshipLink> {
        sqlx::query_as!(
            PersonRelationshipLink,
            "SELECT * FROM person_relationship_link"
        ).fetch_all(self.pool.as_ref()).await.unwrap_or_else(|_| Vec::new())
    }

    async fn save(&self, obj: PersonRelationshipLink) -> Result<PersonRelationshipLink, Error> {
        let new_person_relationship_link = obj.clone();
        let result = sqlx::query(
            "INSERT INTO person_relationship_link (person_id, relationship_id)
                VALUES (?, ?)")
            .bind(obj.person_id)
            .bind(obj.relationship_id)
            .execute(self.pool.as_ref()).await;
        match result {
            Ok(_) => Ok(new_person_relationship_link),
            Err(err) => Err(err)
        }
    }
}