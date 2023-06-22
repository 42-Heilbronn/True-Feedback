use diesel_async::pooled_connection::PoolError;
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};
use diesel_async::AsyncPgConnection;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<AsyncPgConnection>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, PoolError> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        let pool = Pool::builder().build(config).await?;
        Ok(Database { pool })
    }
}
