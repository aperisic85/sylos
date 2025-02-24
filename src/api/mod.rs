pub mod handlers;
pub mod models;
pub mod db;
pub mod index;
pub mod messages;

use axum::Router;
use axum::routing::get;
use crate::api::index::index;
pub fn create_app() -> Router {
    Router::new().route("/messages", get(handlers::show_syslog_messages))
                    .route("/",get( index))
}
