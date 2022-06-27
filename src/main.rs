use csv_parse_server::*;
use db::*;
use http::StatusCode;
use serde::Serialize;
use std::convert::Infallible;
use warp::{Buf, Filter, Rejection, Reply};

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}
// handle the error in processing the post body
pub async fn error_handler(error: Rejection) -> Result<impl Reply, Infallible> {
    println!("error: {:?}", error);

    let (code, response) = if let Some(e) = error.find::<ServerError>() {
        (
            StatusCode::BAD_REQUEST,
            ErrorMessage {
                message: format!("{}", e),
            },
        )
    } else if let Some(e) = error.find::<PgDbError>() {
        (
            StatusCode::BAD_REQUEST,
            ErrorMessage {
                message: format!("{}", e),
            },
        )
    } else {
        println!("can't handle the error");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorMessage {
                message: "unhandled error happend on server".to_string(),
            },
        )
    };
    let reply = warp::reply::json(&response);
    Ok(warp::reply::with_status(reply, code))
}

pub async fn insert_into_db(
    record: &CityRecord,
    db_connection: &DBConnection,
) -> Result<(), PgDbError> {
    match record.population {
        Some(ref popluation) => {
            let statement = "INSERT INTO location (City, State, Population, Latitude, Longitude) VALUES ($1, $2, $3, $4, $5)".to_string();
            db_connection
                .query(
                    &statement,
                    &[
                        &record.city,
                        &record.state,
                        popluation,
                        &record.latitude,
                        &record.longitude,
                    ],
                )
                .await
                .map_err(|source| PgDbError::FailedToExecuteSql { statement, source })?;
        }
        None => {
            let statement =
                "INSERT INTO location (City, State, Latitude, Longitude) VALUES ($1, $2, $3, $4)"
                    .to_string();
            db_connection
                .query(
                    &statement,
                    &[
                        &record.city,
                        &record.state,
                        &record.latitude,
                        &record.longitude,
                    ],
                )
                .await
                .map_err(|source| PgDbError::FailedToExecuteSql { statement, source })?;
        }
    }
    Ok(())
}

pub async fn process_post_body(
    body: impl Buf,
    db_pool: DBConnectionPool,
) -> Result<impl Reply, warp::Rejection> {
    println!("receive post request");
    let db_connection = db::retrieve_db_connection(&db_pool).await?;
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .flexible(true)
        .terminator(csv::Terminator::Any(b'\n'))
        .trim(csv::Trim::All)
        .from_reader(body.chunk());
    let mut response = vec![];
    for result in csv_reader.deserialize() {
        // report client the error when the csv parsing failed or database operation failed
        let record: CityRecord = result.map_err(|e| ServerError::CsvError { source: e })?;
        insert_into_db(&record, &db_connection).await?;
        response.push(record);
    }

    Ok(warp::reply::json(&response))
}

fn pass_db_pool(
    db_pool: DBConnectionPool,
) -> impl Filter<Extract = (DBConnectionPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

#[tokio::main]
async fn main() {
    let db_pool =
        db::create_connection_pool().expect("cant' create a pool for database connection");
    db::init_database(&db_pool)
        .await
        .expect("can't initialize the database");
    let route = warp::path!("upload")
        .and(warp::post())
        .and(warp::body::aggregate())
        .and(pass_db_pool(db_pool.clone()))
        .and_then(process_post_body)
        .recover(error_handler);
    println!("server is running");
    warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}
