
use axum::{
    response::{Json as JsonResponse},
    extract::Json
};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice};


use crate::configs::env::EnvConfig;
use crate::utils::decrypt;
use crate::models::error::ErrorResponse;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    token: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    username: String,
    password: String
}


pub async fn verify(token: &str) -> Result<bool, ErrorResponse> {
    let env_config = EnvConfig::get();
    let admin_username = &env_config.admin_username;
    let admin_password = &env_config.admin_password;

    
    let decrypted = decrypt::new(token)
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    let creds: Credentials = from_slice(&decrypted)
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    if (creds.username == *admin_username) && (creds.password == *admin_password) {
        return Ok(true);
    }else{
        return Ok(false);
    }
}


pub async fn new(Json(payload): Json<Payload>) -> Result<JsonResponse<bool>, ErrorResponse>{
    let result = verify(&payload.token).await?;

    return Ok(JsonResponse(result));
    
}