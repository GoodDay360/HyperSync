use tracing_subscriber::FmtSubscriber;
use tracing::{error};
use tower_http::services::{ServeDir, ServeFile};
use std::net::SocketAddr;
use axum::{
    Router,
    http::Method
};
use tower_governor::{
    governor::GovernorConfigBuilder, GovernorLayer,
    key_extractor::SmartIpKeyExtractor,
};
use tower_http::cors::{CorsLayer, Any};


use tokio::{self, time::{Duration, sleep}};


pub mod entities;
pub mod utils;
pub mod configs;
pub mod models;

pub mod middleware;
pub mod routes;
pub mod methods;






#[tokio::main]
async fn main() {
    /* Initialize Logger */
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    /* --- */

    /* Load Environment Variables For Debug Mode */
    configs::env::EnvConfig::init();
    /* --- */

    


    /* Load Database Connection */
    match utils::database::init().await {
        Ok(_) => {},
        Err(e) => {
            error!("[INIT DATABASE] {}", e);
            std::process::exit(0);
        },
    }
    /* --- */
    
    /* Setup Rest Routes */
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
            tracing::info!("[Default] rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    

    let mut app = Router::new()
        .merge(
            Router::new()
                .nest_service("/admin", ServeFile::new("./dist/index.html"))
                .nest_service("/assets", ServeDir::new("./dist/assets"))
                .layer(GovernorLayer::new(governor_conf))
        );
        
        
        

    app = match routes::rest::new(app.clone()).await {
        Ok(app) => {app},
        Err(e) => {
            error!("[REST ROUTES] {}", e);
            std::process::exit(0);
        },
    };
    /* --- */

    /* Setup Socket Route */
    // let (socket_layer, socket_io) = SocketIo::builder()
    //     .max_payload(3 * 1024)
    //     .build_layer();
    

    // match routes::socket::new(socket_io) {
    //     Ok(_) => {},
    //     Err(e) => {
    //         error!("[SOCKET ROUTES] {}", e);
    //         std::process::exit(0);
    //     },
    // }

    // app = app.layer(socket_layer);

    /* --- */
    

    /* Setup CORS */

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    app = app.layer(cors);
    /* --- */

    /* Spawn Workers */
    
    models::auth_user::AUTH_USER::spawn_worker();
    models::watch_state::CACHE_WATCH_STATE::spawn_worker();

    /* --- */
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
