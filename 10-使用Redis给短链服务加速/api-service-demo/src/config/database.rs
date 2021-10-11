use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use redis::Client;
use redis::aio::Connection;

pub async fn do_connect() -> Pool<MySql> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:jkxsl12369@127.0.0.1/shorten_db").await;
    pool.unwrap()
}

pub async fn do_redis_connect() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    client.get_async_connection().await.unwrap()
}
