use lazy_static::lazy_static;

use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use crate::configstore::CONFIG;
use crate::errors::DBConnectionError;

pub type ConnPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref CONN_MANAGER: ConnPool = {
        let database_url = &CONFIG.database.url;
        let db_manager = ConnectionManager::<PgConnection>::new(database_url);
        ConnPool::builder()
            .build(db_manager)
            .expect("Failed to create pool.")
    };
}

pub fn get_connection() -> Result<PgPooledConnection, DBConnectionError> {
    CONN_MANAGER
        .get()
        .map_err(|e| DBConnectionError(e.to_string()))
}
