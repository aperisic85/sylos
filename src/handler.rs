use crate::syslog::parser;
use std::str; // Import parser module
use tokio::fs::OpenOptions;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::UdpSocket;

pub async fn handle_syslog_messages(socket: &UdpSocket) -> tokio::io::Result<()> {
    let mut buf = [0; 2048];
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("syslog_msg.txt")
        .await?;
    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        let message = str::from_utf8(&buf[..len]).unwrap_or("Invalid UTF-8 message");

        //println!("\nReceived {} bytes from {}", len, src);
        //println!("Raw message: {}", message);

        if let Ok((_rest, parsed_log)) = parser::parse_syslog(message) {
            //println!("Parsed Log: {:?}", parsed_log);
            //filter by severity
            let critical_severities = vec!["Emergency", "Alert", "Crtitical", "Error"];
            if critical_severities.contains(&parsed_log.severity_name.as_str()) {
                let log_entry = format!(
                    "{} - {} - {} - {}\n",
                    parsed_log.timestamp.unwrap_or_default(),
                    parsed_log.severity_name,
                    parsed_log.hostname,
                    parsed_log.message
                );

                // Write the log entry to the file asynchronously
                file.write_all(log_entry.as_bytes()).await?;
                println!("Log saved to file!");
            }
        } else {
            println!("Failed to parse syslog message.");
        }
    }
}
