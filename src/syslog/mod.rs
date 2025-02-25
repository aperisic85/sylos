// syslog.rs - Helper functions for syslog parsing

// Import parser and utils modules for syslog message parsing and priority handling.
pub mod parser;
pub mod utils;
pub mod handler;
use tokio_postgres::{NoTls, Client};
use tokio_postgres::error::Error;

/// A function to process syslog messages and handle them appropriately.
/// This could include logging them, filtering by severity, and more.
/// Currently, the logic is separated into parser and utility functions for modularity.
pub fn process_syslog_message(message: &str) {
    // Example function to process a syslog message
    if let Ok((_rest, parsed_log)) = parser::parse_syslog(message) {
        // Logic for processing parsed log goes here
        println!("Parsed Log: {:?}", parsed_log);
    } else {
        println!("Failed to parse syslog message: {}", message);
    }
}

/// Function to convert a syslog priority integer into a more readable format.
/// This combines the facility and severity to help debug or log the message in an easier-to-read way.
pub fn format_priority(priority: u8) -> String {
    let (_, facility_name, _, severity_name) = utils::get_priority_info(priority);
    format!("{} - {}", facility_name, severity_name)
}

pub async fn create_postgres_client() -> Result<Client,Error> {
    // Replace with your PostgreSQL connection string
    let connection_str = "host=localhost user=postgres password=mypas dbname=template1";
    
    let (client, connection) = tokio_postgres::connect(connection_str, NoTls)
        .await?;

    // Spawn a task to drive the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}