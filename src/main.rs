mod syslog;
mod api;

use axum::{Router};
use std::net::SocketAddr;
use tokio::net::{TcpListener, UdpSocket};
use tokio::task;

async fn run_api_server(app: Router) {
    let adr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = TcpListener::bind(&adr).await.unwrap();
    println!("Starting app...");
    axum::serve(listener, app).await.unwrap();
    println!("Listening for API on {}", adr);
}

async fn run_syslog_handler() {
    let client = syslog::create_postgres_client().await;
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:1514").await.unwrap();
    println!("Listening for syslog messages on {}", socket.local_addr().unwrap());

    syslog::handler::handle_syslog_messages(&socket, &client).await.unwrap();
}

#[tokio::main]
async fn main() {
    let app = api::create_app();

    // Spawn the API server in a new task
    tokio::spawn(run_api_server(app));

    // Spawn the syslog handler in a new task
    tokio::spawn(run_syslog_handler());

    // Let the runtime handle async tasks concurrently
    // Using tokio::task::block_in_place or similar to avoid blocking the main task
    loop {
        tokio::task::yield_now().await;
    }
}
