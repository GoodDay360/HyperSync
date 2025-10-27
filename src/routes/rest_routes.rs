use axum::{
    routing::get,
    Router,
};

pub async fn new(app: Router) -> Result<(), String> {

    let _ = app
        .route("/", get(|| async { "Hello, World!" }));

    return Ok(());
}