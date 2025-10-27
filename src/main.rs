use sea_orm::{DatabaseConnection, Database, Statement, ConnectionTrait};
use lazy_static::lazy_static;
use tracing_subscriber::{fmt, EnvFilter};
use tracing::{error, info};
use axum::{
    routing::get,
    Router,
};


mod routes;
mod utils;


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
    let app = Router::new();
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
    axum::serve(listener, app).await.unwrap();
}
