use axum::{
    response::{Json as JsonResponse},
    extract::{Json},
    http::{HeaderMap},
};

use serde::{Deserialize, Serialize};
use sea_orm::{
    EntityTrait, QueryFilter, QuerySelect, Condition, ColumnTrait
};


use crate::entities::{user};
use crate::utils::database;
use crate::models::error::ErrorResponse;
use crate::methods::rest::{admin};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    page: usize,
    search: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: String,
    email: String,
    username: String,
    status: bool,
    timestamp: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool,
    data: Vec<User>,
}

const LIMIT:usize = 20;

pub async fn new(headers: HeaderMap, Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let mut admin_token_verify: bool = false;
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            
            admin_token_verify = admin::verify::verify(auth_str).await 
                .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;
        }
    }

    if !admin_token_verify {
        return Err(ErrorResponse{status: 500, message: "Invalid admin token.".to_string()});
    }


    let conn = database::get_connection().await
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;


    let users = user::Entity::find()
        .select_only()
        .column(user::Column::Id)
        .column(user::Column::Email)
        .column(user::Column::Username)
        .column(user::Column::Timestamp)
        .column(user::Column::Status)
        .filter(
            Condition::any()
                .add(user::Column::Id.contains(&payload.search))
                .add(user::Column::Email.contains(&payload.search))
                .add(user::Column::Username.contains(&payload.search))
        )
        .limit(LIMIT as u64)
        .offset(((payload.page - 1) * LIMIT) as u64)
        .into_tuple::<(String,String,String,i64, bool)>()
        .all(&conn)
        .await
        .map_err(|e| ErrorResponse { status: 500, message: e.to_string() })?;

    let mut result: Vec<User> = Vec::new();
    for user in users {
        
        let new_user = User{
            id: user.0,
            email: user.1, 
            username: user.2, 
            timestamp: user.3 as usize,
            status: user.4
        };
        result.push(new_user);
    }

    return Ok(JsonResponse(Response{status: true, data: result}));

}