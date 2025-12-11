use crate::crud::person::PersonRepository;
use sqlx::{MySql, Pool};
use std::sync::Arc;
use crate::crud::person_relationship_link::PersonRelationshipLinkRepository;
use crate::crud::relationship::RelationshipRepository;

pub struct AppState {
    pub person_repository: PersonRepository,
    pub person_relationship_link_repository: PersonRelationshipLinkRepository,
    pub relationship_repository: RelationshipRepository,
    pub google_oauth2_client_id: String,
}

impl AppState {
    pub fn new(pool: Pool<MySql>, google_oauth2_client_id: String) -> AppState {
        let pool = Arc::new(pool);

        return AppState {
            person_repository: PersonRepository {
                pool: Arc::clone(&pool)
            },
            person_relationship_link_repository: PersonRelationshipLinkRepository {
                pool: Arc::clone(&pool)
            },
            relationship_repository: RelationshipRepository {
                pool: Arc::clone(&pool)
            },
            google_oauth2_client_id,
        };
    }
}