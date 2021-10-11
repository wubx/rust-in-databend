use axum::handler::{post, get};
use axum::{Router, AddExtensionLayer};
use axum::routing::BoxRoute;
use redis::Client;

use crate::app::controllers::shortlink_controller;
use sqlx::{Pool, MySql};
use redis::aio::Connection;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};

pub fn app(pool: Pool<MySql>, redis_client: Connection) -> Router<BoxRoute> {
    Router::new()
        .route("/", get(|| async { "welcome to use axum!" }))
        .nest("/api", short_links())
        .layer(AddExtensionLayer::new(pool))
        .layer(AddExtensionLayer::new(Arc::new(Mutex::new(redis_client))))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .boxed()
}

pub fn short_links() -> Router<BoxRoute> {
    Router::new()
        .route("/create_shortlink", post(shortlink_controller::create_shortlink))
        .route("/delete_shortlink", post(shortlink_controller::delete_shortlink))
        .route("/:id", get(shortlink_controller::get_shortlink))
        .route("/not_found", get(shortlink_controller::not_found))
        .boxed()
}