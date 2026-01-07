use crate::CadetHubBeResult;
use common::config::ApplicationConfig;
use common::error::CadetHubError;
use common::logger::info;
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use std::str::FromStr;

#[derive(Clone)]
pub(crate) struct Database {
    connection_pool: SqlitePool,
}

impl Database {
    pub async fn connect(config: &ApplicationConfig) -> CadetHubBeResult<Self> {
        let connection_pool = init_db_connection_pool(config).await?;
        Ok(Self { connection_pool })
    }

    pub(crate) fn share_pool(&self) -> &SqlitePool {
        &self.connection_pool
    }
}

async fn init_db_connection_pool(config: &ApplicationConfig) -> CadetHubBeResult<SqlitePool> {
    let service_name = config.service_name();
    let data_directory_path = config.data_directory_path()?;
    let db_url = config.database().url(data_directory_path)?;

    info!("Initiate DB connection pool: {db_url}");
    let mut options = SqliteConnectOptions::from_str(db_url.as_str())
        .map_err(CadetHubError::general_error_with_source)?
        .create_if_missing(true);
    if let Some(key) = config.database().encryption_key(&service_name)? {
        options = options.pragma("key", format!("'{key}'"))
    }
    let sql_pool = SqlitePool::connect_with(options)
        .await
        .map_err(CadetHubError::general_error_with_source)?;
    info!(
        "DB connection pool initiated: number_of_connections={}, number_of_idele_connections={}",
        sql_pool.size(),
        sql_pool.num_idle()
    );

    run_schema_migration(&sql_pool).await?;

    Ok(sql_pool)
}

async fn run_schema_migration(sql_pool: &Pool<Sqlite>) -> CadetHubBeResult<()> {
    info!("Start DB schema migration");
    sqlx::migrate!("../../crates/backend/migrations")
        .run(sql_pool)
        .await
        .map_err(CadetHubError::general_error_with_source)?;
    info!("Finish DB schema migration");
    Ok(())
}
