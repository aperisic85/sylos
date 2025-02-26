use axum::{Router, routing::get, routing::post};
use std::net::SocketAddr;
use tokio::net::{TcpListener, UdpSocket};
use tokio::task;
use tokio::signal;
use tracing::{info, error};

mod syslog;
mod api;

async fn run_api_server(app: Router) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let adr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = TcpListener::bind(&adr).await?;
    info!("API Server listening on {}", adr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_syslog_handler() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = syslog::create_postgres_client().await?;
    let socket = UdpSocket::bind("0.0.0.0:1514").await?;
    info!("Syslog listener bound to {}", socket.local_addr()?);
    
    syslog::handler::handle_syslog_messages(&socket, &client).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // Initialize logging

    let app = api::create_app();

    let api_task = tokio::spawn(run_api_server(app));
    let syslog_task = tokio::spawn(run_syslog_handler());
    let shutdown_task = tokio::spawn(async {
        signal::ctrl_c().await.unwrap();
        info!("Received shutdown signal, exiting...");
    });

    tokio::select! {
        _ = api_task => error!("API Server crashed"),
        _ = syslog_task => error!("Syslog handler crashed"),
        _ = shutdown_task => {
            info!("Shutting down gracefully...");
        }
    }
}
