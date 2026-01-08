use crate::models::relationship::Relationship;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use crate::crud::repository::Repository;
use crate::models::person_relationship_link::PersonRelationshipLink;

#[utoipa::path(
    post,
    path = "/relationship",
    tag = "relationship",
    request_body = Relationship,
    responses(
        (status = StatusCode::CREATED, description = "Created new relationship", body = Relationship),
        (status = StatusCode::BAD_REQUEST, description = "Less than two ids of people (people_ids) given", body = String),
        (status = StatusCode::UNAUTHORIZED, description = "Bearer Token is missing or invalid"),
        (status = StatusCode::UNPROCESSABLE_ENTITY, description = "Field is missing", body = String),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Error during persisting", body = String),
    ),
)]
pub async fn post(
    State(state): State<Arc<AppState>>,
    Json(relationship): Json<Relationship>,
) -> Result<(StatusCode, Json<Relationship>), (StatusCode, String)> {
    if relationship.people_ids.len() < 2 {
        return Err((StatusCode::BAD_REQUEST, String::new()));
    }

    return match state.relationship_repository.save(relationship).await {
        Ok(relationship) => {
            let links: Vec<PersonRelationshipLink> = relationship.people_ids.iter().map(|p| PersonRelationshipLink {
                person_id: *p,
                relationship_id: relationship.id.unwrap(),
            }).collect();

            return match state.person_relationship_link_repository.save_all(links).await {
                Ok(_) => Ok((StatusCode::CREATED, Json(relationship))),
                Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
            };
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };
}