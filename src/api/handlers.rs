use axum::Json;
use crate::api::db::get_db_client;
use crate::api::models::SyslogMessage;
use tokio_postgres::Client;

pub async fn show_syslog_messages() -> impl axum::response::IntoResponse {
    let client = get_db_client().await;
    let messages = fetch_syslog_messages(&client).await;
    Json(messages)
}


async fn fetch_syslog_messages(client: &Client) -> Vec<SyslogMessage> {
    let query = "SELECT timestamp, severity, hostname, message, sender_ip, sender_port FROM syslog_messages ORDER BY timestamp DESC LIMIT 50";
    let rows = client.query(query, &[]).await.unwrap();

    let mut messages = Vec::new();
    for row in rows {
        let message = SyslogMessage {
            timestamp: row.get("timestamp"),
            severity: row.get("severity"),
            hostname: row.get("hostname"),
            message: row.get("message"),
            sender_ip: row.get("sender_ip"),
            sender_port: row.get("sender_port"),
        };
        messages.push(message);
    }

    messages
}
