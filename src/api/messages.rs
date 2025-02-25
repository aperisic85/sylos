use tokio_postgres::{NoTls, Client};

use crate::api::models::SyslogMessage;
pub async fn fetch_messages() -> Vec<SyslogMessage> {
    let (client, connection) = tokio_postgres::connect(
        "host=127.0.0.1 user=postgres password=mypas dbname=template1",
        NoTls,
    )
    .await
    .expect("Failed to connect to the database");

    // Spawn a connection task to handle async queries
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT timestamp, hostname, severity, message, sender_ip, sender_port FROM syslog_messages", &[])
        .await
        .expect("Failed to fetch logs");

    // Map each row into SyslogMessage struct
    rows.into_iter()
        .map(|row| SyslogMessage {
            timestamp: row.get(0),
            hostname: row.get(1),
            severity: row.get(2),
            message: row.get(3),
            sender_ip: row.get(4),    // Fetch sender_ip
            sender_port: row.get(5),  // Fetch sender_port
        })
        .collect()
}
