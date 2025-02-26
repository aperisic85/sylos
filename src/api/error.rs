use axum::http::StatusCode;
use axum::response::IntoResponse;
use askama_axum::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    error_message: String,
}

pub async fn error_handler() -> Result<impl IntoResponse, StatusCode> {
    let template = ErrorTemplate {
        error_message: "An unexpected error has occurred. Please try again later.".to_string(),
    };
    let html = template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(html))
}
