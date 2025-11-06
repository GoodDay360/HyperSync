use std::f32::consts::E;

use base64::{engine::general_purpose, Engine as _};

use axum::{
    response::{Json as JsonResponse},
    extract::{Json},
    http::{StatusCode, HeaderMap},
};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, json};
use sea_orm::{
    EntityTrait, QueryFilter, ColumnTrait,
    ActiveValue::Set,
};
use orion::aead;
use orion::kdf::SecretKey;
use rand::random;
use chrono::Utc;
use uuid::Uuid;


use crate::entities::user;
use crate::utils::database;
use crate::configs::env::EnvConfig;
use crate::models::error::ErrorResponse;
use crate::utils::encrypt;
use crate::methods::rest::{admin};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    email: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool,
}


pub async fn new(headers: HeaderMap, Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let mut admin_token_verify: bool = false;
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            println!("auth_str: {}", auth_str);
            admin_token_verify = admin::verify::verify(auth_str).await 
                .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;
        }
    }

    if !admin_token_verify {
        return Err(ErrorResponse{status: 500, message: "Invalid admin token.".to_string()});
    }

    let id = Uuid::now_v7().to_string().replace("-", "");

    let creds = json!({
        "id": &id,
        "email": &payload.email,
        "password": &payload.password
    });

    let creds_to_string = to_string(&creds)
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    let token = encrypt::new(&creds_to_string.as_bytes())
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    let encrypt_password = encrypt::new(&payload.password.as_bytes())
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    let new_user = user::ActiveModel {
        id: Set(id),
        email: Set(payload.email),
        username: Set(payload.username),
        password: Set(encrypt_password),
        token: Set(token),
        timestamp: Set(Utc::now().timestamp_millis()),
    };

    let conn = database::get_connection().await
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    user::Entity::insert(new_user)
        .exec(&conn)
        .await
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    return Ok(JsonResponse(Response{status: true}));

}