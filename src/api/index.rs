use askama_axum::Template;
use axum::{extract::Query, response::IntoResponse, response::Html};
use serde::Deserialize;
use crate::api::messages::fetch_messages;
use crate::api::models::SyslogMessage;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    messages: Vec<SyslogMessage>,
}

#[derive(Deserialize)]
pub struct LoadMessages {
    load: Option<String>,
}

pub async fn index(query: Query<LoadMessages>) -> impl IntoResponse {
    let messages = if query.load.is_some() {
        fetch_messages().await
    } else {
        Vec::new()
    };

    let template = IndexTemplate { messages };
    Html(template.render().unwrap())
}
