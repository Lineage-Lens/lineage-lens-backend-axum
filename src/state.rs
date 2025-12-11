use crate::crud::person::PersonRepository;
use sqlx::{MySql, Pool};
use std::sync::Arc;

pub struct AppState {
    pub person_repository: PersonRepository,
}

impl AppState {
    pub fn new(pool: Pool<MySql>) -> AppState {
        let pool = Arc::new(pool);

        return AppState {
            person_repository: PersonRepository {
                pool
            }
        };
    }
}