mod person;
mod relationship;

use crate::state::AppState;
use axum::http::{header, HeaderValue, Method};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use axum::middleware;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::middleware::auth::authorize;

pub async fn start_server(ip_addr: IpAddr, port: u16, state: AppState) {
    #[derive(OpenApi)]
    #[openapi(paths(
        person::get::get,
        person::post::post,
        relationship::post::post,
    ))]
    struct ApiDoc;

    let state = Arc::new(state);
    let state1 = Arc::clone(&state);
    let state2 = Arc::clone(&state1);

    let person_router = person::router(state1);
    let relationship_router = relationship::router(state2);

    let app = person_router
        .merge(relationship_router)
        .layer(create_cors_layer())
        .layer(middleware::from_fn(move |r, n| authorize(Arc::clone(&state), r, n)))
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()));

    let address = SocketAddr::new(ip_addr, port);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn create_cors_layer() -> CorsLayer {
    let allowed_origins = std::env::var("ALLOWED_ORIGINS").unwrap();
    let allowed_origins: Vec<String> = serde_json::from_str(allowed_origins.as_str()).unwrap();

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins.iter().map(|e| HeaderValue::from_str(e.as_str()).unwrap())))
        .allow_credentials(true)
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
}