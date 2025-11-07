use axum::{
    response::{Json as JsonResponse},
    extract::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{to_string};


use crate::configs::env::EnvConfig;
use crate::models::error::ErrorResponse;
use crate::utils::encrypt;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool,
    token: String
}


pub async fn new(Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let env_config = EnvConfig::get();

    let admin_username = &env_config.admin_username;
    let admin_password = &env_config.admin_password;

    if (payload.username == *admin_username) && (payload.password == *admin_password) {
        
        let cred_to_string = to_string(&payload)
            .map_err(|e| ErrorResponse {
                status: 500,
                message: e.to_string(),
            })?;
        
        let token: String = encrypt::new(cred_to_string.as_bytes())
            .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;
        
        return Ok(JsonResponse(Response{
            status: true, 
            token: token
        }));
    }else{
        return Err(ErrorResponse {
                status: 500,
                message: "Invalid username or password.".to_string(),
            });
    }
}