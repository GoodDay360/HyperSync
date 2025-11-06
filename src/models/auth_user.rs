
use lazy_static::lazy_static;
use dashmap::DashMap;
use tokio::{self, time::{Duration, sleep}};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};

use crate::entities::user;
use crate::utils::database;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserState {
    pub user_id: String,
    pub last_use_count: usize,
    pub last_use_timestamp: usize,
    pub timestamp: usize
}

lazy_static! {
    // <user_token, UserState>
    pub static ref AUTH_USER: DashMap<String, UserState> = DashMap::new();
}

const MAX_LIFE: usize = 5 * 60 * 1000;

const MAX_PER_USE_COUNT: usize = 5;
const MAX_PER_USE_INTERVAL: usize = 10 * 1000;


impl AUTH_USER {
    pub fn spawn_worker() {
        tokio::spawn(async move {
            loop {
                AUTH_USER.retain(|_, v| (Utc::now().timestamp_millis() as usize) - (*v).timestamp <= MAX_LIFE);
                sleep(Duration::from_secs(5)).await;
            }
        });
    }
    
    pub fn verify(token: &str) -> Result<Option<UserState>, String> {
        if let Some(mut auth_user) = AUTH_USER.get_mut(token) {
            let current_timpstamp = Utc::now().timestamp_millis() as usize;
            if (current_timpstamp - (*auth_user).timestamp) <= MAX_LIFE {
                if current_timpstamp - (*auth_user).last_use_timestamp <= MAX_PER_USE_INTERVAL {
                    if (*auth_user).last_use_count < MAX_PER_USE_COUNT {
                        (*auth_user).last_use_count += 1;
                        info!("last use count: {}", (*auth_user).last_use_count);
                        return Ok(Some(auth_user.clone()));
                    }else{
                        error!("[auth_user:verify] exceed token max per use count.");
                        return Ok(None);
                    }
                }else{
                    (*auth_user).last_use_count = 0;
                    (*auth_user).last_use_timestamp = current_timpstamp;
                    return Ok(Some(auth_user.clone()));
                }
            }
        }

        error!("[auth_user:verify] Invalid token.");
        return Err("Invalid token.".to_string())?;
    }

    pub async fn add(token: &str) -> Result<UserState, String>{
        let conn = database::get_connection().await
            .map_err(|e| e.to_string())?;
        let result = user::Entity::find()
            .select_only()
            .column(user::Column::Id)
            .filter(user::Column::Token.eq(token))
            .into_tuple::<String>()
            .one(&conn).await
            .map_err(|e| e.to_string())?;

        if let Some(user_id) = result {
            let current_timestamp = Utc::now().timestamp_millis() as usize;
            let new_user_state = UserState{
                user_id,
                last_use_count: 0,
                last_use_timestamp: current_timestamp,
                timestamp: current_timestamp
            };
            AUTH_USER.insert(token.to_string(), new_user_state.clone());
            return Ok(new_user_state);
        }else{
            error!("[auth_user:add] Invalid token.");
            return Err("[auth_user:add] Invalid token.".to_string())?;
        }
    }
}
