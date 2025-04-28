use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::Connection;
use diesel::PgConnection;
use domain::errors::Kind::Internal;
use domain::AppResult;
use std::env;

pub struct SessionManager {
    pool: Pool<ConnectionManager<PgConnection>>,
}
impl SessionManager {
    pub fn new(database_url: &str) -> AppResult<Self> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .map_err(Internal.from_srcf())?;

        Ok(Self { pool })
    }

    pub fn from_env() -> AppResult<Self> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| Internal.with("DATABASE_URLが設定されていません"))?;
        Self::new(&database_url)
    }

    pub fn get_connection(&self) -> AppResult<PooledConnection<ConnectionManager<PgConnection>>> {
        self.pool.get().map_err(Internal.from_srcf())
    }

    pub fn transaction<F, T>(&self, f: F) -> AppResult<T>
    where
        F: FnOnce(&mut PgConnection) -> AppResult<T>,
    {
        let mut conn = self.get_connection()?;
        conn.transaction(|v| match f(v) {
            Ok(val) => Ok(val),
            Err(_) => Err(diesel::result::Error::RollbackTransaction),
        })
        .map_err(Internal.from_srcf())
    }

    pub fn read<F, T>(&self, f: F) -> AppResult<T>
    where
        F: FnOnce(&mut PgConnection) -> AppResult<T>,
    {
        let mut conn = self.get_connection()?;
        f(&mut conn)
    }
}
