use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

pub async fn do_connect() -> Pool<MySql> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:jkxsl12369@127.0.0.1/shorten_db").await;
       // .connect("mysql://sulin:databenD!9@localhost:3306/shorten_db").await;
    pool.unwrap()
}
