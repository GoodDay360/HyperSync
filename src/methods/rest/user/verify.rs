
use axum::{
    response::{Json as JsonResponse},
    extract::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};


use crate::entities::user;
use crate::utils::database;
use crate::models::error::ErrorResponse;
use crate::utils::decrypt;
use crate::models::user::UserCredentials;

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
        .select_only()
        .column(user::Column::Password)
        .filter(user::Column::Token.eq(&payload.token))
        .into_tuple::<String>()
        .one(&conn)
        .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    if let Some(raw_password) = result {
        let creds: UserCredentials = from_slice(
            &decrypt::new(&payload.token)
            .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?
        ).map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

        let password = String::from_utf8(decrypt::new(&raw_password)
            .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?
        ).map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

        if creds.password == password {
            return Ok(JsonResponse(Response{status: true}));
        }else{
            return Ok(JsonResponse(Response{status: false}));
        }
        
    }else{
        return Ok(JsonResponse(Response{status: false}))
    }

}