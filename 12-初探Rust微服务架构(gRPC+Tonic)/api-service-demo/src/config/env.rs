// use sqlx::{Pool, MySql};
// use redis::aio::Connection;
// use std::sync::Arc;
//
// #[derive(Clone, Debug)]
// pub struct Environment {
//     mysql_conn: Pool<MySql>,
//     redis_conn: Arc::new<Connection>
// }
//
// impl Environment {
//     pub async fn new(mysql_conn: Pool<MySql>, redis_conn: Arc::new<Connection>) -> anyhow::Result<Self> {
//         Ok(Self {
//             mysql_conn,
//             redis_conn
//         })
//     }
//
//     pub fn db(self) -> Pool<MySql> {
//         self.mysql_conn
//     }
//
//     pub fn clients(self) -> Arc::new<Connection> {
//         self.redis_conn
//     }
// }