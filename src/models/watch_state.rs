use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value};

use serde_json::{to_string};
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Condition,
    Set
};
use sea_query::{OnConflict}; 

use tracing::{info, error};
use lazy_static::lazy_static;
use dashmap::DashMap;
use chrono::Utc;
use tokio::{self, time::{Duration, sleep}};
use uuid::Uuid;

use crate::entities::{user, watch_state};
use crate::utils::database;
use crate::configs::env::EnvConfig;
use crate::models::error::ErrorResponse;
use crate::utils::decrypt;

use crate::models::auth_user::AUTH_USER;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WatchState {
    pub current_time: f64,
    pub timestamp: usize
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CacheInfo {
    pub info: DashMap<
        String, // source
        DashMap<
            String, // id
            DashMap<
                usize, // season_index
                DashMap<
                    usize, // episode_index
                    WatchState
                >
            >
        >
    >,
    pub timestamp: usize
}


lazy_static! {
    pub static ref CACHE_WATCH_STATE: DashMap<
        String, // user_id
        CacheInfo
    > = DashMap::new();
}

impl CACHE_WATCH_STATE {
    
    pub fn spawn_worker() {
        tokio::spawn(async move { 
            loop {
                let current_timestamp = Utc::now().timestamp_millis() as usize;
                let cloned_cache_watch_state = CACHE_WATCH_STATE.clone();

                for watch_state in cloned_cache_watch_state.iter() {
                    let user_id = watch_state.key().to_string();

                    for source_info in watch_state.value().info.iter() {
                        let source = source_info.key();

                        for id_info in source_info.value().iter() {
                            let id = id_info.key();

                            for season_index_info in id_info.value().iter() {
                                let season_index = *season_index_info.key();

                                for episode_index_info in season_index_info.value().iter() {
                                    let episode_index = *episode_index_info.key();

                                    let current_time = episode_index_info.value().current_time;
                                    let timestamp = episode_index_info.value().timestamp;

                                    match upload_watch_state(
                                        &user_id,
                                        &source,
                                        &id,
                                        season_index,
                                        episode_index,
                                        current_time,
                                        timestamp
                                    ).await {
                                        Ok(_) => {}
                                        Err(e) => {
                                            error!("[CACHE_WATCH_STATE] {}", e);
                                        }
                                    }

                                }
                            }
                        }
                    }
                }
                
                CACHE_WATCH_STATE.retain(|_, value| value.timestamp > current_timestamp);
                sleep(Duration::from_secs(10)).await;
            }
        });
    }

    pub fn add(
        user_id: &str,
        source: &str,
        id: &str,
        season_index: usize,
        episode_index: usize,
        current_time: f64,
        timestamp: usize
    ){
        let current_timestamp = Utc::now().timestamp_millis() as usize;
        let new_cache_info = CacheInfo { info: DashMap::new(), timestamp: current_timestamp };

        new_cache_info.info.insert(source.to_string(), DashMap::new());

        if let Some(mut_id) = new_cache_info.info.get_mut(source) {
            mut_id.insert(id.to_string(), DashMap::new());

            if let Some(mut_season_index) = mut_id.get_mut(id) {
                mut_season_index.insert(season_index, DashMap::new());
                if let Some(mut_episode_index) = mut_season_index.get_mut(&season_index) {
                    
                    mut_episode_index.insert(episode_index, WatchState{current_time, timestamp});
                }
            }
        }
        
        CACHE_WATCH_STATE.insert(user_id.to_string(), new_cache_info);
        
    }
}


pub async fn upload_watch_state(
    user_id: &str,
    source: &str,
    id: &str,
    season_index: usize,
    episode_index: usize,
    current_time: f64,
    timestamp: usize
) -> Result<(), String> {
    let conn = database::get_connection().await?;

    let result = watch_state::Entity::find()
        .select_only()
        .column(watch_state::Column::Timestamp)
        .filter(
            Condition::all()
                .add(watch_state::Column::UserId.eq(user_id.to_string()))
                .add(watch_state::Column::Source.eq(source.to_string()))
                .add(watch_state::Column::Id.eq(id.to_string()))
                .add(watch_state::Column::SeasonIndex.eq(season_index as i32))
                .add(watch_state::Column::EpisodeIndex.eq(episode_index as i32))
        )
        .into_tuple::<i64>()
        .one(&conn)
        .await.map_err(|e| e.to_string())?;

    if let Some(stored_timestamp) = result {
        if stored_timestamp as usize > timestamp {
            return Ok(());
        }
    }

    let new_watch_state = watch_state::ActiveModel{
        watch_state_id: Set(Uuid::now_v7().to_string().replace("-", "")),
        user_id: Set(user_id.to_string()),
        source: Set(source.to_string()),
        id: Set(id.to_string()),
        season_index: Set(season_index as i32),
        episode_index: Set(episode_index as i32),
        current_time: Set(current_time),
        timestamp: Set(timestamp as i64)
    };

    watch_state::Entity::insert(new_watch_state)
        .on_conflict(
            OnConflict::columns([
                watch_state::Column::UserId,
                watch_state::Column::Source,
                watch_state::Column::Id,
                watch_state::Column::SeasonIndex,
                watch_state::Column::EpisodeIndex,
            ])
                .update_columns([
                    watch_state::Column::CurrentTime,
                    watch_state::Column::Timestamp
                ])
                .to_owned()
        )
        .exec(&conn)
        .await.map_err(|e| e.to_string())?;
    return Ok(());
}