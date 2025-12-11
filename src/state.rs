use crate::crud::person::PersonRepository;
use sqlx::{MySql, Pool};
use std::sync::Arc;

pub struct AppState {
    pub person_repository: PersonRepository,
    pub google_oauth2_client_id: String,
}

impl AppState {
    pub fn new(pool: Pool<MySql>, google_oauth2_client_id: String) -> AppState {
        let pool = Arc::new(pool);

        return AppState {
            person_repository: PersonRepository {
                pool
            },
            google_oauth2_client_id,
        };
    }
}