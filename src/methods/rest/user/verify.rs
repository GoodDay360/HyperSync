
use axum::{
    response::{Json as JsonResponse},
    extract::Json,
};
use serde::{Deserialize, Serialize};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};


use crate::entities::user;
use crate::utils::database;
use crate::models::error::ErrorResponse;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool
}


pub async fn new(Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let conn = database::get_connection().await
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    let result = user::Entity::find()
        .filter(user::Column::Token.eq(&payload.token))
        .count(&conn)
        .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    if result > 0 {
        return Ok(JsonResponse(Response{status: true}))
    }else{
        return Ok(JsonResponse(Response{status: false}))
    }

}