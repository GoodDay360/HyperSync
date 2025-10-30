use base64::{engine::general_purpose, Engine as _};

use axum::{
    response::{Json as JsonResponse},
    extract::Json
};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, from_slice};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use orion::aead;
use orion::kdf::SecretKey;
use rand::random;

use crate::entities::user;
use crate::utils::database;
use crate::configs::env::EnvConfig;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    token: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    username: String,
    password: String
}


async fn verify(token: &str) -> Result<bool, String> {
    let env_config = EnvConfig::get();
    let secret = env_config.secret.as_bytes();
    let admin_username = &env_config.admin_username;
    let admin_password = &env_config.admin_password;

    let key = SecretKey::from_slice(secret).map_err(|e| e.to_string())?;

    let decoded = general_purpose::STANDARD.decode(token).map_err(|e| e.to_string())?;
    let decrypted = aead::open(&key, &decoded).map_err(|e| e.to_string())?;

    let creds: Credentials = from_slice(&decrypted).map_err(|e| e.to_string())?;

    if (creds.username == *admin_username) && (creds.password == *admin_password) {
        return Ok(true);
    }else{
        return Ok(false);
    }
}


pub async fn new(Json(payload): Json<Payload>) -> Result<JsonResponse<bool>, String>{
    let result = verify(&payload.token).await?;

    return Ok(JsonResponse(result));
    
}