pub mod handlers;
pub mod models;
pub mod db;

use axum::Router;
use axum::routing::get;

pub fn create_app() -> Router {
    Router::new().route("/messages", get(handlers::show_syslog_messages))
}
