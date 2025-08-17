use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct DBClient {
    pub pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        return Self { pool };
    }
}
