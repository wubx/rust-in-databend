use std::error::Error;
use std::net::SocketAddr;

mod config;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let pool = config::database::do_connect().await;
    let redis_client = config::database::do_redis_connect().await;
    axum::Server::bind(&addr)
        .serve(config::routes::app(pool, redis_client).into_make_service())
        .await?;
    Ok(())
}