use base64::{engine::general_purpose, Engine as _};

use axum::{
    response::{Json as JsonResponse},
    extract::Json,
    http::StatusCode
};
use serde::{Deserialize, Serialize};
use serde_json::{to_string};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QuerySelect};
use orion::aead;
use orion::kdf::SecretKey;
use rand::random;

use crate::entities::user;
use crate::utils::database;
use crate::configs::env::EnvConfig;
use crate::models::error::ErrorResponse;
use crate::utils::decrypt;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool,
    token: String
}


pub async fn new(Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    
    let conn = database::get_connection().await
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    let result = user::Entity::find()
        .select_only()
        .column(user::Column::Password)
        .column(user::Column::Token)
        .filter(user::Column::Email.eq(&payload.email))
        .into_tuple::<(String, String)>()
        .one(&conn)
        .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    if let Some((password, token)) = result {
        let decrypt_password = String::from_utf8(decrypt::new(&password)?)
            .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

        if decrypt_password == payload.password {
            return Ok(JsonResponse(Response{status: true, token: token}));
        }else{
            return Err(ErrorResponse{status: 500, message: "Invalid email or password.".to_string()});
        }
    }else{
        return Err(ErrorResponse{status: 500, message: "User not found.".to_string()});
    }

}