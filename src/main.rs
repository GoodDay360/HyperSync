use sea_orm::{DatabaseConnection, Database, Statement, ConnectionTrait};
use lazy_static::lazy_static;
use tracing_subscriber::{fmt, EnvFilter};
use tracing::{error, info};
use tower_http::services::{ServeDir, ServeFile};
use std::net::SocketAddr;
use axum::{
    Router,
};
use tower_governor::{
    governor::GovernorConfigBuilder, GovernorLayer,
    key_extractor::SmartIpKeyExtractor,
};
use tokio::{self, time::Duration};

pub mod entities;
pub mod utils;
pub mod configs;

mod routes;
mod rest_methods;





#[tokio::main]
async fn main() {
    /* Initialize Logger */
    fmt()
        .with_env_filter(EnvFilter::new("info")) 
        .init();
    /* --- */

    /* Load Environment Variables For Debug Mode */
    #[cfg(debug_assertions)]
    {
        use dotenv::dotenv;
        dotenv().ok();
        info!("[dotenv] Loaded .env in debug mode");
    }
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
    
    /* Setup Routes */
    let governor_conf = GovernorConfigBuilder::default()
        .period(Duration::from_secs(10))
        .burst_size(30)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    let governor_limiter = governor_conf.limiter().clone();
    
    tokio::spawn(async move {
        let interval = Duration::from_secs(60);
        loop {
            std::thread::sleep(interval);
            tracing::info!("[Default] rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    let app = Router::new()
        .merge(
            Router::new()
                .route_service("/", ServeFile::new("./dist/index.html"))
                .nest_service("/assets", ServeDir::new("./dist/assets"))
                .layer(GovernorLayer::new(governor_conf))
        );
        
        

    match routes::rest_routes::new(app.clone()).await {
        Ok(_) => {},
        Err(e) => {
            error!("[REST ROUTES] {}", e);
            std::process::exit(0);
        },
    }
    // match routes::socket_routes::new(app.clone()).await {
    //     Ok(_) => {},
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         std::process::exit(0);
    //     },
    // }
    /* --- */

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
