use crate::state::AppState;
use axum::Router;
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;

pub async fn start_server(ip_addr: IpAddr, port: u16, state: AppState) {
    let app = Router::new();

    let address = SocketAddr::new(ip_addr, port);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}