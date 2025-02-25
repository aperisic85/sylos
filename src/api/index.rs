use axum::http::StatusCode;
use axum::response::IntoResponse;
use askama_axum::Template;
use crate::api::messages::fetch_messages;
use crate::api::models::SyslogMessage;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    messages: Vec<SyslogMessage>,
}

pub async fn index() -> Result<impl IntoResponse, StatusCode> {
    let messages = fetch_messages().await;
    let template = IndexTemplate { messages };
    let html = template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(html))
}
