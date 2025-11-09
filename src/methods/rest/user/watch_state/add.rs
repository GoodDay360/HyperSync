use axum::{
    response::{Json as JsonResponse},
    extract::Json,
    http::{HeaderMap},
};
use serde::{Deserialize, Serialize};

use crate::models::user::{AUTH_USER};
use crate::models::error::ErrorResponse;
use crate::models::watch_state::{CACHE_WATCH_STATE};



#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Payload {
    source: String,
    id: String,
    season_index: usize,
    episode_index: usize,
    current_time: f64,
    timestamp: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool
}


pub async fn new(headers: HeaderMap, Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let token = headers.get("authorization")
        .ok_or("Missing user token.")
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?
        .to_str()
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?;
    
    let user_state = AUTH_USER::verify(&token)
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?
        .ok_or("Unable to use this token.")
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?;

    CACHE_WATCH_STATE::add(
        &user_state.user_id,
        &payload.source,
        &payload.id,
        payload.season_index,
        payload.episode_index,
        payload.current_time,
        payload.timestamp
    );

    return Ok(JsonResponse(Response{status: true}));
}
