mod handler;
mod syslog; // Import handler module

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:1515").await.unwrap();
    println!("Listening on {}", socket.local_addr().unwrap());

    handler::handle_syslog_messages(&socket).await.unwrap();
}
