use axum::{
    response::{Json as JsonResponse},
    extract::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value};

use serde_json::{to_string};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};
use chrono::Utc;

use crate::entities::user;
use crate::utils::database;
use crate::models::auth_user::{AUTH_USER, UserState};
use crate::models::error::ErrorResponse;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool
}



pub async fn auth_user(Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let conn = database::get_connection().await
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;
    let result = user::Entity::find()
        .select_only()
        .column(user::Column::Id)
        .filter(user::Column::Token.eq(&payload.token))
        .into_tuple::<String>()
        .one(&conn).await
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    if let Some(user_id) = result {
        let current_timestamp = Utc::now().timestamp_millis() as usize;
        
        AUTH_USER.insert(payload.token.clone(), UserState{
            user_id,
            last_use_count: 0,
            last_use_timestamp: current_timestamp,
            timestamp: current_timestamp
        });
        return Ok(JsonResponse(Response{status: true}));
    }else{
        return Ok(JsonResponse(Response{status: false}));
    }
}
