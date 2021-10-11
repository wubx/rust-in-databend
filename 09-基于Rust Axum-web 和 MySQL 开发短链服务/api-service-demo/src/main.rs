use std::error::Error;
use std::net::SocketAddr;

mod config;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let pool = config::database::do_connect().await;

    axum::Server::bind(&addr)
        .serve(config::routes::app(pool).into_make_service())
        .await?;
    Ok(())
}