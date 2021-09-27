use sqlx::MySqlPool;

#[derive(Clone, Debug)]
pub struct Environment {
    pool: MySqlPool
}

impl Environment {
    pub async fn new(pool: MySqlPool) -> anyhow::Result<Self> {
        Ok(Self {
            pool
        })
    }
}
