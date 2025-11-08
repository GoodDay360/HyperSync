use axum::{
    response::{Json as JsonResponse},
    extract::{Json},
    http::{HeaderMap},
};
use serde::{Deserialize, Serialize};
use sea_orm::{
    EntityTrait, QueryFilter, ColumnTrait
};


use crate::entities::{user, favorite, watch_state};
use crate::utils::database;
use crate::models::error::ErrorResponse;
use crate::methods::rest::{admin};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    data: Vec<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool,
}


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

    user::Entity::delete_many()
        .filter(user::Column::Id.is_in(&payload.data))
        .exec(&conn)
        .await
        .map_err(|e| ErrorResponse { status: 500, message: e.to_string() })?;

    favorite::Entity::delete_many()
        .filter(favorite::Column::UserId.is_in(&payload.data))
        .exec(&conn)
        .await
        .map_err(|e| ErrorResponse { status: 500, message: e.to_string() })?;

    watch_state::Entity::delete_many()
        .filter(watch_state::Column::UserId.is_in(&payload.data))
        .exec(&conn)
        .await
        .map_err(|e| ErrorResponse { status: 500, message: e.to_string() })?;

    return Ok(JsonResponse(Response{status: true}));

}