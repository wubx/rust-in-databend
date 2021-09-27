use axum::handler::{post, get};
use axum::{Router, AddExtensionLayer};
use axum::routing::BoxRoute;

use crate::config::env::Environment;
use crate::app::controllers::shortlink_controller;

pub fn app(env: Environment) -> Router<BoxRoute> {
    Router::new()
        .route("/", get(|| async { "welcome to use axum!" }))
        .nest("/api", short_links())
        .layer(AddExtensionLayer::new(env))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .boxed()
}

pub fn short_links() -> Router<BoxRoute> {
    Router::new()
        .route("/create_shortlink", post(shortlink_controller::create_shortlink))
        .route("/delete_shortlink", post(shortlink_controller::delete_shortlink))
        .boxed()
}