use crate::models::person::Person;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use crate::crud::repository::Repository;

pub async fn post(
    State(state): State<Arc<AppState>>,
    Json(mut person): Json<Person>,
) -> Result<(StatusCode, Json<Person>), (StatusCode, String)> {
    person.id = None;
    let result = state.person_repository.save(person).await;
    return match result {
        Ok(person) => Ok((StatusCode::CREATED, Json::from(person))),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string()))
    };
}