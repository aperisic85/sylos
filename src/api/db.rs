use tokio_postgres::{Client, NoTls};

pub async fn get_db_client() -> Client {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres dbname=syslog_db password=mypas", NoTls)
        .await
        .expect("Failed to connect to the database");

    // Spawn the connection to the database in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error during database connection: {}", e);
        }
    });

    client
}
