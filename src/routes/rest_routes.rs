use axum::{
    routing::get,
    Router,
};

use crate::rest_methods;

pub async fn new(app: Router) -> Result<(), String> {

    let _ = app
        .route("/login", get(rest_methods::login::new));

    return Ok(());
}