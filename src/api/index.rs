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

pub async fn index() -> impl IntoResponse {
    let messages = fetch_messages().await;
    let template = IndexTemplate { messages };
    //template.into_response()
      // Render the template into a String and return it wrapped in an Html response
      Html(template.render().unwrap())
}
