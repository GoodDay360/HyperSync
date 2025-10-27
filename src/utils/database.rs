use sea_orm::{DatabaseConnection, Database};
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::env;


lazy_static! {
    static ref DATABASE: DashMap<usize, DatabaseConnection> = DashMap::new();
}

pub async fn init() -> Result<(), String> {
    get_connection().await?;
    
    return Ok(());
}

pub async fn get_connection() -> Result<DatabaseConnection, String> {

    if let Some(db) = DATABASE.get(&0) {
        return Ok(db.clone());
    }
    let db_uri = env::var("DATABASE_URI").map_err(|e| e.to_string())?;
    let new_conn = Database::connect(&db_uri).await.map_err(|err| err.to_string())?;
    DATABASE.insert(0, new_conn.clone());
    return Ok(new_conn);

}




