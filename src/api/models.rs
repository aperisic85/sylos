use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct SyslogMessage {
    pub timestamp: String,
    pub severity: String,
    pub hostname: String,
    pub message: String,
    pub sender_ip: String,
    pub sender_port: i32,
}
