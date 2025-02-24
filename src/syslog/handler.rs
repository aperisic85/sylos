use crate::syslog::parser;
use std::str; // Import parser module
use tokio::fs::OpenOptions;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::UdpSocket;
use tokio_postgres::{Client, NoTls};

pub async fn handle_syslog_messages(socket: &UdpSocket, client: &Client) -> tokio::io::Result<()> {
    let mut buf = [0; 2048];
    
    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        let message = str::from_utf8(&buf[..len]).unwrap_or("Invalid UTF-8 message");

        println!("\nReceived {} bytes from {}", len, src);
        println!("Raw message: {}", message);

        if let Ok((_rest, parsed_log)) = parser::parse_syslog(message) {
            // Filter by severity (critical)
            //let critical_severities = vec!["Emergency", "Alert", "Critical", "Error"];
           // if critical_severities.contains(&parsed_log.severity_name.as_str()) {
                let timestamp = parsed_log.timestamp.unwrap_or_default();
                let severity = parsed_log.severity_name;
                let hostname = parsed_log.hostname;
                let message = parsed_log.message;
                let sender_ip = src.ip().to_string(); // Get sender's IP

                // Cast sender port (u16) to i32 for PostgreSQL compatibility
                let sender_port = src.port() as i32;

                // Insert the parsed data into PostgreSQL
                client
                    .execute(
                        r#"
                            INSERT INTO syslog_messages (timestamp, severity, hostname, message, sender_ip, sender_port)
                            VALUES ($1, $2, $3, $4, $5, $6)
                        "#,
                        &[
                            &timestamp.to_rfc3339(),
                            &severity,
                            &hostname,
                            &message,
                            &sender_ip,
                            &sender_port,
                        ],
                    )
                    .await
                    .expect("Failed to insert syslog message into PostgreSQL");

                println!("Log saved to PostgreSQL!");
            }
        //} else {
           // println!("Failed to parse syslog message.");
        //}
    }
}