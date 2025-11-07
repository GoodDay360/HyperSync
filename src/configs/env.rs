
use std::env;
use dashmap::DashMap;
use tracing::{info};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Arc;



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvConfig{
    pub secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

lazy_static! {
    static ref ENV_CONFIG: DashMap<usize, Arc<EnvConfig>> = DashMap::new();
}

impl EnvConfig {
    
    pub fn init() {
        #[cfg(debug_assertions)]
        {
            use dotenv::dotenv;
            dotenv().ok();
            info!("[dotenv] Loaded .env in debug mode");
        }

        ENV_CONFIG.insert(0, Arc::new(EnvConfig {
            secret: env::var("SECRET").expect("[env] missing 'SECRET'"),
            admin_username: env::var("ADMIN_USERNAME").expect("[env] missing 'ADMIN_USERNAME'"),
            admin_password: env::var("ADMIN_PASSWORD").expect("[env] missing 'ADMIN_PASSWORD'"),
        }));
    }

    pub fn get() -> Arc<EnvConfig> {
        return ENV_CONFIG.get(&0).unwrap().value().clone();
    }

}