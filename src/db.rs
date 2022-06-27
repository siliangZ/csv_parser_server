use crate::{DBConnection, DBConnectionPool};
use mobc::Pool;
use mobc_postgres::PgConnectionManager;
use snafu::{ResultExt, Snafu};
use std::str::FromStr;
use tokio_postgres::{Config, Error as tokio_postgres_error, NoTls};

const DP_POOL_MAX_OPEN: u64 = 16;
const DP_POOL_MAX_IDEL: u64 = 8;
const DP_POOL_TIMEOUT_SEC: u64 = 20;

#[derive(Snafu, Debug)]
pub enum PgDbError {
    #[snafu(display("can't parse config from str"))]
    #[snafu(context(false))]
    ConfigErr { source: tokio_postgres_error },
    #[snafu(display("can't get a valid connection to db, error: {}", source))]
    #[snafu(context(false))]
    FailedToGetConnection {
        source: mobc::Error<tokio_postgres_error>,
    },
    #[snafu(display("can't read init sql from {}", init_sql_path))]
    FailedToParseInitSql { init_sql_path: String },

    #[snafu(display("can't execute {}, error: {}", statement, source))]
    FailedToExecuteSql {
        statement: String,
        source: tokio_postgres_error,
    },
}
impl warp::reject::Reject for PgDbError {}

type Result<T> = std::result::Result<T, PgDbError>;

pub fn create_connection_pool() -> Result<DBConnectionPool> {
    let config =
        Config::from_str("host=127.0.0.1 user=siliang port=5050 password=000000 dbname=postgres")?;
    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DP_POOL_MAX_OPEN)
        .max_idle(DP_POOL_MAX_IDEL)
        .get_timeout(Some(std::time::Duration::from_secs(DP_POOL_TIMEOUT_SEC)))
        .build(manager))
}

pub async fn retrieve_db_connection(db_pool: &DBConnectionPool) -> Result<DBConnection> {
    Ok(db_pool.get().await?)
}

pub async fn init_database(db_pool: &DBConnectionPool) -> Result<()> {
    let init_sql_path = "init_db.sql".to_string();
    let init_sql = std::fs::read_to_string(&init_sql_path)
        .map_err(|_| PgDbError::FailedToParseInitSql { init_sql_path })?;
    let conn = retrieve_db_connection(db_pool).await?;
    //[debug] renew the table for debugging
    let statement = "DROP TABLE IF EXISTS location".to_string();
    conn.execute(&statement, &[])
        .await
        .context(FailedToExecuteSqlSnafu { statement })?;
    // initialize the database

    conn.batch_execute(&init_sql).await.unwrap();
    Ok(())
}
