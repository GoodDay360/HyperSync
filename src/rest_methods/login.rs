use axum::{response::{Response}};

use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

use crate::entities::user;
use crate::utils::database;
pub async fn new() -> Result<Response, String>{
    
    let conn = database::get_connection().await?;

    let user_query: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Id.eq("123"))
        .one(&conn)
        .await.map_err(|e| e.to_string())?;

    if let Some(user) = user_query {
        println!("{}", user.username);
        
    }else{
        return Err("User not found.".into())
    }

    return Err("Unhandled error occured.".into())
}