use axum::{
    response::{Json as JsonResponse},
    extract::Json,
    http::{HeaderMap},
};
use serde::{Deserialize, Serialize};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, query::JoinType, sea_query::{Expr},
};
use serde_json::{json};
use uuid::Uuid;

use crate::entities::{favorite::{self}, user};
use crate::utils::database;
use crate::models::error::ErrorResponse;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
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
}


pub async fn new(headers: HeaderMap, Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let token = headers.get("authorization")
        .ok_or("Missing user token.")
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?
        .to_str()
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?;


    let conn = database::get_connection().await
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    let result = favorite::Entity::find()
        .join(
            JoinType::InnerJoin,
            favorite::Entity::belongs_to(user::Entity)
                .from(favorite::Column::UserId)
                .to(user::Column::Id)
                .into(),
        )
        .select_only()
        .column(favorite::Column::FavoriteId)
        .filter(user::Column::Token.eq(token))
        .filter(favorite::Column::Source.eq(&payload.source))
        .filter(favorite::Column::Id.eq(&payload.id))
        .into_tuple::<String>()
        .one(&conn)
        .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    if let Some(favorite_id) = result {
        favorite::Entity::update_many()
            .col_expr(favorite::Column::Tags, Expr::value(json!(&payload.tags)))
            .col_expr(favorite::Column::CurrentWatchSeasonIndex, Expr::value(payload.current_watch_season_index))
            .col_expr(favorite::Column::CurrentWatchEpisodeIndex, Expr::value(payload.current_watch_episode_index))
            .col_expr(favorite::Column::Timestamp, Expr::value(payload.timestamp as i64))
            .filter(favorite::Column::FavoriteId.eq(favorite_id))
            .filter(favorite::Column::Timestamp.lt(payload.timestamp as i64))
            .exec(&conn)
            .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;
        return Ok(JsonResponse(Response{status: true}));
    }else{
        let user_result = user::Entity::find()
            .select_only()
            .column(user::Column::Id)
            .filter(user::Column::Token.eq(token))
            .into_tuple::<String>()
            .one(&conn)
            .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

        if let Some(user_id) = user_result {
            favorite::ActiveModel {
                favorite_id: Set(Uuid::now_v7().to_string().replace("-", "")),
                user_id: Set(user_id),
                source: Set(payload.source),
                id: Set(payload.id),
                tags: Set(json!(&payload.tags)),
                current_watch_season_index: Set(payload.current_watch_season_index),
                current_watch_episode_index: Set(payload.current_watch_episode_index),
                timestamp: Set(payload.timestamp as i64),
            }
                .insert(&conn)
                .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

            return Ok(JsonResponse(Response{status: true}));
        }else{
            return Err(ErrorResponse{status: 500, message: "Authentication failed.".to_string()});
        }

    }
    
        

}