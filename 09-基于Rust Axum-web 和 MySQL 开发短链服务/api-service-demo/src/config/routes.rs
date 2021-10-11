use axum::handler::{post, get};
use axum::{Router, AddExtensionLayer};
use axum::routing::BoxRoute;

use crate::app::controllers::shortlink_controller;
use sqlx::{Pool, MySql};

pub fn app(pool: Pool<MySql>) -> Router<BoxRoute> {
    Router::new()
        .route("/", get(|| async { "welcome to use axum!" }))
        .nest("/api", short_links())
        .layer(AddExtensionLayer::new(pool))
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