use socketioxide::{
    SocketIo,
};

use crate::methods::socket::watch_state::on_connect_watch_state;



pub fn new(socket_io: SocketIo) -> Result<(), String> {

    socket_io.ns("/socket/user/watch_state", on_connect_watch_state);
        
    return Ok(());
}