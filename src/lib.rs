use mobc::{Connection, Pool};
use mobc_postgres::PgConnectionManager;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use tokio_postgres::NoTls;
pub mod db;
pub type DBConnection = Connection<PgConnectionManager<NoTls>>;
pub type DBConnectionPool = Pool<PgConnectionManager<NoTls>>;

#[derive(Snafu, Debug)]
pub enum ServerError {
    #[snafu(display("{}", source))]
    #[snafu(context(false))]
    DBError { source: db::PgDbError },

    #[snafu(display("{}", source))]
    #[snafu(context(false))]
    CsvError { source: csv::Error },
}
impl warp::reject::Reject for ServerError {}

#[derive(Deserialize, Serialize, Debug)]
pub struct CityRecord {
    pub city: String,
    pub state: String,
    pub population: Option<i32>,
    pub latitude: f32,
    pub longitude: f32,
}
