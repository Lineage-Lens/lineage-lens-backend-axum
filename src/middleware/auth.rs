use std::sync::Arc;
use axum::body::Body;
use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use axum::http::{Method, Response};
use axum::middleware::Next;
use serde_json::{Map, Value};
use crate::state::AppState;

pub async fn authorize(state: Arc<AppState>, req: Request, next: Next) -> Result<Response<Body>, String> {
    if req.method() == Method::OPTIONS {
        return Ok(next.run(req).await);
    }

    let auth = req.headers().get(AUTHORIZATION);
    if auth.is_none() || auth.unwrap().to_str().is_err() || !auth.unwrap().to_str().unwrap().starts_with("Bearer ") {
        return create_unauthorized();
    }

    let auth = auth.unwrap().to_str().unwrap().replace("Bearer ", "");

    let mut url = "https://oauth2.googleapis.com/tokeninfo?id_token=".to_owned();
    url.push_str(auth.as_str());

    let res = reqwest::get(url).await;

    if res.is_err() {
        return create_unauthorized();
    }

    let res = res.unwrap();

    if !res.status().is_success() {
        return create_unauthorized();
    }

    let data: Value = serde_json::from_str(res.text().await.unwrap().as_str()).unwrap();
    let data: Map<String, Value> = data.as_object().unwrap().clone();

    match data.get("aud") {
        Some(data) => {
            if data.eq(state.google_oauth2_client_id.as_str()) {
                Ok(next.run(req).await)
            } else {
                create_unauthorized()
            }
        }
        None => create_unauthorized()
    }
}

fn create_unauthorized() -> Result<Response<Body>, String> {
    Err(String::new())
}