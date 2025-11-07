use axum::{
    response::{Json as JsonResponse},
    extract::Json,
    http::{HeaderMap},
};
use serde::{Deserialize, Serialize};
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, query::JoinType
};
use serde_json::{from_value, Value};

use crate::entities::{user, favorite};
use crate::utils::database;

use crate::models::error::ErrorResponse;




#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Payload {
    page: usize,
    timestamp: usize
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Favorite {
    source: String,
    id: String,
    tags: Vec<String>,
    current_watch_season_index: Option<i32>,
    current_watch_episode_index: Option<i32>,
    timestamp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool,
    data: Vec<Favorite>,
}

const LIMIT: usize = 15;

pub async fn new(headers: HeaderMap, Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let token = headers.get("authorization")
        .ok_or("Missing user token.")
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?
        .to_str()
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?;

    let offset = (payload.page - 1) * LIMIT;


    let conn = database::get_connection().await
        .map_err(|e| ErrorResponse { status: 500, message: e.to_string() })?;
    

    let result = favorite::Entity::find()
        .join(
            JoinType::InnerJoin,
            favorite::Entity::belongs_to(user::Entity)
                .from(favorite::Column::UserId)
                .to(user::Column::Id)
                .into(),
        )
        .select_only()
        .column(favorite::Column::Source)
        .column(favorite::Column::Id)
        .column(favorite::Column::Tags)
        .column(favorite::Column::CurrentWatchSeasonIndex)
        .column(favorite::Column::CurrentWatchEpisodeIndex)
        .column(favorite::Column::Timestamp)
        .filter(user::Column::Token.eq(token))
        .filter(favorite::Column::Timestamp.gte(payload.timestamp as i64))
        .offset(offset as u64)
        .limit(LIMIT as u64)
        .order_by_asc(favorite::Column::Timestamp)
        .into_tuple::<(String, String, Value, Option<i32>, Option<i32>, i64)>()
        .all(&conn)
        .await.map_err(|e| ErrorResponse { status: 500, message: e.to_string() })?
        .into_iter()
        .map(|(source, id, tags, current_season_index, current_episode_index, timestamp)| 
            Favorite {
                source,
                id,
                tags: from_value(tags).map_err(|e| ErrorResponse { status: 500, message: e.to_string() }).unwrap_or(vec![]),
                current_watch_season_index: current_season_index,
                current_watch_episode_index: current_episode_index,
                timestamp: timestamp as usize,
            }
        )
        .collect::<Vec<Favorite>>();
    
    return Ok(JsonResponse(Response{status: true, data: result}));
    
}
