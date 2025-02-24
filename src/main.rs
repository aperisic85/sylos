mod syslog; // Import the syslog module

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:1515").await.unwrap();
    println!("Listening on {}", socket.local_addr().unwrap());

    syslog::handler::handle_syslog_messages(&socket).await.unwrap();
}
