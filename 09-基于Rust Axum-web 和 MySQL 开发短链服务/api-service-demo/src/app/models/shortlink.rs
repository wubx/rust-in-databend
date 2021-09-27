use sqlx::{Error, MySql, Pool, FromRow};
use sqlx::mysql::MySqlQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct ShortLink {
    pub id: i32,
    pub url: String,
}

pub async fn create_shortlink(pool: &Pool<MySql>, url: &str) -> Result<MySqlQueryResult, Error> {
    sqlx::query(
        r#"
            INSERT INTO short_links (`url`)
            VALUES(?)"#,
    )
        .bind(url)
        .execute(pool).await
}

pub async fn delete_shortlink(pool: &Pool<MySql>, id: u64) -> Result<MySqlQueryResult, Error> {
    sqlx::query(
        r#"
            DELETE FROM short_links
            WHERE id = ?
            "#,
    )
        .bind(id)
        .execute(pool).await
}
