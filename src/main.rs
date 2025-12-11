mod api;
mod crud;
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

    api::start_server(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port, AppState::new(pool)).await;
}