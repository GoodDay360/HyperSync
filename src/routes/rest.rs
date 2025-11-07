use axum::{
    routing::{post},
    Router,
    middleware,
};
use tower_http::limit::RequestBodyLimitLayer;

use tower_governor::{
    governor::GovernorConfigBuilder, GovernorLayer,
    key_extractor::SmartIpKeyExtractor,
};

use tokio::{self, time::{Duration, sleep}};

use crate::methods::rest::{admin, user};
use crate::middleware::auth_user;

pub async fn new(app: Router) -> Result<Router, String> {

    /* Rate Limit Middleware */
    let governor_conf = GovernorConfigBuilder::default()
        .period(Duration::from_secs(5))
        .burst_size(30)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    let governor_limiter = governor_conf.limiter().clone();
    
    tokio::spawn(async move {
        let interval = Duration::from_secs(60);
        loop {
            sleep(interval).await;
            tracing::info!("[REST ROUTE] rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });
     /* --- */

    let new_app = app.nest("/api", Router::new()
        .nest("/admin", Router::new()
            .route("/login", post(admin::login::new))
            .route("/verify", post(admin::verify::new))
            .route("/create_user", post(admin::create_user::new))
        )
        .nest("/user", Router::new()
            .route("/login", post(user::login::new))
            .route("/change_password", post(user::change_password::new))
            .nest("/favorite", Router::new()
                .nest("/add", Router::new()
                    .route("/", post(user::favorite::add::new))
                    .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024)) 
                )
                .nest("/get", Router::new()
                    .route("/", post(user::favorite::get::new)) 
                    .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) 
                )
                .route_layer(middleware::from_fn(auth_user::new))
            )
            .nest("/watch_state", Router::new()
                .route("/add", post(user::watch_state::add::new))
                .route("/get", post(user::watch_state::get::new))
                .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
                .route_layer(middleware::from_fn(auth_user::new))
            )
        )
    )
    .layer(RequestBodyLimitLayer::new(8 * 1024 * 1024))
    .layer(GovernorLayer::new(governor_conf));
        
    return Ok(new_app);
}