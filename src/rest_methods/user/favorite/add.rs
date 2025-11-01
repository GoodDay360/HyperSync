
use base64::{engine::general_purpose, Engine as _};

use axum::{
    response::{Json as JsonResponse},
    extract::Json,
    http::{StatusCode, HeaderMap},
};
use serde::{Deserialize, Serialize};
use serde_json::{to_string};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, query::JoinType, sea_query::{Expr, OnConflict}
};
use serde_json::{json};
use uuid::Uuid;
use chrono::Utc;

use crate::entities::{favorite::{self, Entity}, user};
use crate::utils::database;
use crate::configs::env::EnvConfig;
use crate::models::error::ErrorResponse;
use crate::utils::decrypt;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    source: String,
    id: String,
    tags: Vec<String>,
    current_watch_season_index: usize,
    current_watch_episode_index: usize,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool,
}


pub async fn new(headers: HeaderMap, Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let mut token:String = "".to_string();
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            token = auth_str.to_string();
            println!("auth_str: {}", auth_str);
        }
    }

    if token.is_empty() {
        return Err(ErrorResponse{status: 500, message: "Missing user token.".to_string()});
    }


    let conn = database::get_connection().await
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    if payload.tags.len() > 0 {
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
            .filter(user::Column::Token.eq(&token))
            .filter(favorite::Column::Source.eq(&payload.source))
            .filter(favorite::Column::Id.eq(&payload.id))
            .into_tuple::<String>()
            .one(&conn)
            .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

        if let Some(favorite_id) = result {
            favorite::Entity::update_many()
                .col_expr(favorite::Column::Tags, Expr::value(json!(&payload.tags)))
                .col_expr(favorite::Column::CurrentWatchSeasonIndex, Expr::value(payload.current_watch_season_index as i32))
                .col_expr(favorite::Column::CurrentWatchEpisodeIndex, Expr::value(payload.current_watch_episode_index as i32))
                .filter(favorite::Column::FavoriteId.eq(favorite_id))
                .exec(&conn)
                .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;
            return Ok(JsonResponse(Response{status: true}));
        }else{
            let user_result = user::Entity::find()
                .select_only()
                .column(user::Column::Id)
                .filter(user::Column::Token.eq(&token))
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
                    current_watch_season_index: Set(payload.current_watch_season_index as i32),
                    current_watch_episode_index: Set(payload.current_watch_episode_index as i32),
                    timestamp: Set(Utc::now().timestamp_millis()),
                }
                    .insert(&conn)
                    .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

                return Ok(JsonResponse(Response{status: true}));
            }else{
                return Err(ErrorResponse{status: 500, message: "Authentication failed.".to_string()});
            }

        }
    }else{
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
            .filter(user::Column::Token.eq(&token))
            .filter(favorite::Column::Source.eq(&payload.source))
            .filter(favorite::Column::Id.eq(&payload.id))
            .into_tuple::<String>()
            .one(&conn)
            .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

        if let Some(favorite_id) = result {
            favorite::Entity::delete_many()
                .filter(favorite::Column::FavoriteId.eq(favorite_id))
                .exec(&conn)
                .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;
        }

        return Ok(JsonResponse(Response{status: true}));
    }
        

}