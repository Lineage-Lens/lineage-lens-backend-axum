mod api;
mod crud;
mod middleware;
mod models;
mod state;

use crate::state::AppState;
use std::net::{IpAddr, Ipv4Addr};
use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

    let url = std::env::var("DATABASE_URL").unwrap();
    let pool = MySqlPoolOptions::new().max_connections(10).connect(&url).await.unwrap();

    let google_oauth2_client_id = std::env::var("GOOGLE_OAUTH2_CLIENT_ID").unwrap();

    api::start_server(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port, AppState::new(pool, google_oauth2_client_id)).await;
}