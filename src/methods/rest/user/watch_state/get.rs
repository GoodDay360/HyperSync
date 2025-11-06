use std::f32::consts::E;

use axum::{
    response::{Json as JsonResponse},
    extract::Json,
    http::{StatusCode, HeaderMap},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value};

use serde_json::{to_string};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, query::JoinType, sea_query::{Expr, OnConflict},
    Condition
};
use chrono::Utc;

use crate::entities::{user, watch_state};
use crate::utils::database;
use crate::models::auth_user::{AUTH_USER, UserState};
use crate::models::error::ErrorResponse;
use crate::models::watch_state::{CACHE_WATCH_STATE};



#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Payload {
    source: String,
    id: String,
    season_index: usize,
    episode_index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool,
    current_time: f64,
    timestamp: usize,
}

pub async fn new(headers: HeaderMap, Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let token = headers.get("authorization")
        .ok_or("Missing user token.")
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?
        .to_str()
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?;


    let conn = database::get_connection().await
        .map_err(|e| ErrorResponse { status: 500, message: e.to_string() })?;

    let result = watch_state::Entity::find()
        .join(
            JoinType::InnerJoin,
            watch_state::Entity::belongs_to(user::Entity)
                .from(watch_state::Column::UserId)
                .to(user::Column::Id)
                .into(),
        )
        .select_only()
        .column(watch_state::Column::CurrentTime)
        .column(watch_state::Column::Timestamp)
        .filter(user::Column::Token.eq(token))
        .filter(
            Condition::all()
                .add(watch_state::Column::Source.eq(payload.source))
                .add(watch_state::Column::Id.eq(payload.id))
                .add(watch_state::Column::SeasonIndex.eq(payload.season_index as i32))
                .add(watch_state::Column::EpisodeIndex.eq(payload.episode_index as i32))
        )
        .into_tuple::<(f64, i64)>()
        .one(&conn)
        .await.map_err(|e| ErrorResponse { status: 500, message: e.to_string() })?;
    
    if let Some((current_time, timestamp)) = result {
        return Ok(JsonResponse(Response{status: true, current_time, timestamp: timestamp as usize}));
    }else{
        return Err(ErrorResponse{status: 404, message: "Watch state not found.".to_string()});
    }
}
