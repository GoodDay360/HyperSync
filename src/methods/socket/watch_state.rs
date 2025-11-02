use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value};

use serde_json::{to_string};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};
use tracing::{info, error};
use lazy_static::lazy_static;
use dashmap::DashMap;

use crate::entities::user;
use crate::utils::database;
use crate::configs::env::EnvConfig;
use crate::models::error::ErrorResponse;
use crate::utils::decrypt;

use crate::models::{
    auth_user::AUTH_USER,
    watch_state::{CACHE_WATCH_STATE, WatchState}
};



#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Auth{
    token: String,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WatchStatePayload {
    user_token: String,
    source: String,
    id: String,
    season_index: usize,
    episode_index: usize,
    current_time: f64,
    timestamp: usize
}


pub async fn on_connect_watch_state(socket: SocketRef, Data(data): Data<Auth>) {
    info!("data: {:?}", data);

    /* Handshake to verify token */
    match AUTH_USER::verify(&data.token) {
        Ok(_) => {
            info!("connected");
            socket.emit("authoriziation", &true).ok();
        }
        Err(_) => {
            socket.emit("authoriziation", &false).ok();
            let _ = socket.clone().disconnect();
        }
    }
    /* --- */


    socket.on("update", async |socket: SocketRef, Data::<WatchStatePayload>(data)| {
        info!("data: {:?}", data);

        let user_state = match AUTH_USER::verify(&data.user_token) {
            Ok(user_state) => {user_state},
            Err(_) => {
                socket.emit("authoriziation", &false).ok();
                let _ = socket.clone().disconnect();
                return;
            }
        };

        CACHE_WATCH_STATE::add(
            &user_state.user_id,
            &data.source,
            &data.id,
            data.season_index,
            data.episode_index,
            data.current_time,
            data.timestamp
        );
    });
}