use crate::models::relationship::{Relationship, RelationshipType};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDate;
use serde::Deserialize;
use std::sync::Arc;
use crate::crud::repository::Repository;
use crate::models::person_relationship_link::PersonRelationshipLink;

#[derive(Deserialize)]
pub struct CreateRelationship {
    relationship_type: RelationshipType,
    start_date: NaiveDate,

    people: Vec<i32>,
}

pub async fn post(
    State(state): State<Arc<AppState>>,
    Json(relationship_dto): Json<CreateRelationship>,
) -> Result<(StatusCode, Json<Relationship>), (StatusCode, String)> {
    let relationship = Relationship {
        id: None,
        relationship_type: relationship_dto.relationship_type,
        start_date: relationship_dto.start_date,
    };

    if relationship_dto.people.is_empty() {
        return Err((StatusCode::BAD_REQUEST, String::new()));
    }

    return match state.relationship_repository.save(relationship).await {
        Ok(relationship) => {
            let links: Vec<PersonRelationshipLink> = relationship_dto.people.iter().map(|p| PersonRelationshipLink {
                person_id: *p,
                relationship_id: relationship.id.unwrap(),
            }).collect();

            return match state.person_relationship_link_repository.save_all(links).await {
                Ok(_) => Ok((StatusCode::CREATED, Json(relationship))),
                Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
            };
        },
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    };
}