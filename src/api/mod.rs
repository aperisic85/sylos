pub mod device;
pub mod models;
pub mod db;
pub mod index;
pub mod messages;
pub mod handlers;

use axum::{Router,Extension};
use axum::routing::{get, post};
use crate::api::index::index;
use crate::syslog;


pub fn create_app() -> Router {

    Router::new().route("/messages", get(handlers::show_syslog_messages))
                    .route("/",get( index))
                    .route("/devices/new", get(device::new_device_form))
                    .route("/error", get(device::dummy_error))
}
