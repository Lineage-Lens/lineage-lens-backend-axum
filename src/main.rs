mod api;

use std::net::{IpAddr, Ipv4Addr};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

    api::start_server(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port).await;
}