use axum::{
    routing::{get, post},
    Router,
};

use crate::methods::rest::{admin, user};

pub async fn new(app: Router) -> Result<Router, String> {

    let new_app = app.nest("/api", Router::new()
        .nest("/admin", Router::new()
            .route("/login", post(admin::login::new))
            .route("/verify", post(admin::verify::new))
            .route("/create_user", post(admin::create_user::new))
        )
        .nest("/user", Router::new()
            .route("/login", post(user::login::new))
            .nest("/favorite", Router::new()
                .route("/add", post(user::favorite::add::new))
            )
            .nest("/watch_state", Router::new()
                .route("/auth_user", post(user::watch_state::auth_user))
            )
        )
        
    );
        
    return Ok(new_app);
}