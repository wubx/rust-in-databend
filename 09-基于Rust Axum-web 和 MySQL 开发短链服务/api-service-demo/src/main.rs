use std::error::Error;
use std::net::SocketAddr;

mod config;
mod dto;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let connect = config::database::do_connect().await;

    axum::Server::bind(&addr)
        .serve(config::routes::app(config::env::Environment::new(connect).await?).into_make_service())
        .await?;
    Ok(())
}