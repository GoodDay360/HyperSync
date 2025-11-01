use base64::{engine::general_purpose, Engine as _};
use orion::aead;
use orion::kdf::SecretKey;


use crate::configs::env::EnvConfig;
use crate::models::error::ErrorResponse;

pub fn new(data: &str) -> Result<Vec<u8>, ErrorResponse> {
    let env_config = EnvConfig::get();

    let secret = env_config.secret.as_bytes();

    let key = SecretKey::from_slice(secret)
        .map_err(|e| ErrorResponse {
            status: 500,
            message: format!("[Decrypt] from_slice {}", e.to_string()),
        })?;

    
    let decoded = general_purpose::STANDARD.decode(data)
        .map_err(|e| ErrorResponse {
            status: 500,
            message: format!("[Decrypt] decode {}", e.to_string()),
        })?;
        
    let result = aead::open(&key, &decoded)
        .map_err(|e| ErrorResponse {
            status: 500,
            message: format!("[Decrypt] open {}", e.to_string()),
        })?;

    return Ok(result);
}