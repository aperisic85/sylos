use axum::response::{Html, IntoResponse};
use axum::extract::{Form, Extension};
use askama_axum::Template;
use serde::Deserialize;
use tokio_postgres::Client;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate;

#[derive(Template)]
#[template(path = "device.html")]
struct DeviceTemplate;

#[derive(Deserialize)]
pub struct NewDevice {
    name: String,
    ip_address: String,
}

pub async fn new_device_form() -> impl IntoResponse {
    Html(DeviceTemplate.render().unwrap())
}

pub async fn create_device(
    Form(new_device): Form<NewDevice>,
    Extension(client): Extension<Client>,
) -> impl IntoResponse {
    let query = "INSERT INTO devices (name, ip_address) VALUES ($1, $2)";
    if let Err(e) = client.execute(query, &[&new_device.name, &new_device.ip_address]).await {
        eprintln!("Failed to insert new device: {}", e);
        return Html(ErrorTemplate.render().unwrap());    }
    Html("Device created successfully".to_string())
}

pub async fn dummy_error() -> impl IntoResponse {
    Html(ErrorTemplate.render().unwrap())
}