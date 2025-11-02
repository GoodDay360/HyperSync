use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value};

use serde_json::{to_string};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};
use orion::aead;
use orion::kdf::SecretKey;
use rand::random;

use crate::entities::user;
use crate::utils::database;
use crate::configs::env::EnvConfig;
use crate::models::error::ErrorResponse;
use crate::utils::decrypt;
use crate::methods::socket::watch_state::on_connect_watch_state;



pub fn new(socket_io: SocketIo) -> Result<(), String> {

    socket_io.ns("/socket/user/watch_state", on_connect_watch_state);
        
    return Ok(());
}