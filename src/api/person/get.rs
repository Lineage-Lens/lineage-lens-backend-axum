use crate::models::person::Person;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use crate::crud::repository::Repository;

pub async fn get(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Person>>, (StatusCode, String)> {
    return Ok(Json::from(state.person_repository.find_all().await));
}