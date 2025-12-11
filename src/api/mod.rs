mod person;

use crate::state::AppState;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn start_server(ip_addr: IpAddr, port: u16, state: AppState) {
    let state1 = Arc::new(state);

    let person_router = person::router(state1);

    let app = person_router;

    let address = SocketAddr::new(ip_addr, port);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}