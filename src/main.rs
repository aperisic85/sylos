mod syslog; // Import the syslog module

use tokio::net::UdpSocket;
use tokio_postgres::Client;

#[tokio::main]
async fn main() {

    let client = syslog::create_postgres_client().await;
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:1514").await.unwrap();
    println!("Listening on {}", socket.local_addr().unwrap());

    syslog::handler::handle_syslog_messages(&socket, &client).await.unwrap();
}
