use axum::{
    routing::{get, post},
    Router,
};

use crate::rest_methods::{admin};

pub async fn new(app: Router) -> Result<Router, String> {

    let new_app = app.nest("/api", Router::new()
        .nest("/admin", Router::new()
            .route("/login", post(admin::login::new))
            .route("/verify", post(admin::verify::new))
        )
        
    );
        
    return Ok(new_app);
}