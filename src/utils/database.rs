use sea_orm::{DatabaseConnection, Database, ConnectOptions};
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::env;
use tokio::{time::Duration};

use migration::{Migrator, MigratorTrait};


lazy_static! {
    static ref DATABASE: DashMap<usize, DatabaseConnection> = DashMap::new();
}

pub async fn init() -> Result<(), String> {
    let conn = get_connection().await?;

    Migrator::up(&conn, None).await.map_err(|e| e.to_string())?;
    
    return Ok(());
}

pub async fn get_connection() -> Result<DatabaseConnection, String> {

    if let Some(db) = DATABASE.get(&0) {
        return Ok(db.clone());
    }
    let db_uri = env::var("DATABASE_URI").map_err(|e| e.to_string())?;

    let mut options = ConnectOptions::new(db_uri);
    options
        .max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(30))
        .sqlx_logging(true);

    let new_conn = Database::connect(options).await.map_err(|e| e.to_string())?;

    DATABASE.insert(0, new_conn.clone());
    return Ok(new_conn);

}






