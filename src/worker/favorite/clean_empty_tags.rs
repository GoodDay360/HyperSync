use chrono::{Utc, Duration};
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter, ConnectionTrait, DatabaseBackend,
    sea_query::{Expr}
};
use tracing::{error, info};
use tokio::{self, time};

use crate::entities::{favorite::{self}};
use crate::utils::database;


pub async fn new() -> Result<(), String>{
    let three_months_ago = Utc::now() - Duration::days(30);
    let threshold_timestamp = three_months_ago.timestamp_millis();

    let conn = database::get_connection().await
        .map_err(|e| e.to_string())?;

    let expr = match conn.get_database_backend() {
        DatabaseBackend::Postgres => Expr::cust("jsonb_array_length(tags)").eq(0),
        DatabaseBackend::MySql => Expr::cust("JSON_LENGTH(tags)").eq(0),
        DatabaseBackend::Sqlite => Expr::cust("json_array_length(tags)").eq(0),
    };

    let row = favorite::Entity::delete_many()
        .filter(favorite::Column::Timestamp.lt(threshold_timestamp))
        .filter(expr)
        .exec(&conn)
        .await
        .map_err(|e| e.to_string())?;

    info!("[worker:favorite:clean_empty_tags] Deleted {} rows with empty tags.", row.rows_affected);
    return Ok(());
}

pub fn spawn_worker() {
    tokio::spawn(async move {
        loop {
            match new().await {
                Ok(_) => {info!("[worker:favorite:clean_empty_tags] success.")},
                Err(e) => error!("[worker:favorite:clean_empty_tags] {}", e),
            }
            time::sleep(time::Duration::from_secs(60)).await;
        }
    });
}