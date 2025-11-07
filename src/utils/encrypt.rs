use base64::{engine::general_purpose, Engine as _};
use orion::aead;
use orion::kdf::SecretKey;


use crate::configs::env::EnvConfig;

pub fn new(data: &[u8]) -> Result<String, String> {
    let env_config = EnvConfig::get();

    let secret = env_config.secret.as_bytes();

    let key = SecretKey::from_slice(secret)
        .map_err(|e| format!("[Encrypt] from_slice {}", e.to_string()))?;

    
    let result_bytes = aead::seal(&key, data)
        .map_err(|e| format!("[Encrypt] seal {}", e.to_string()))?;

    let result: String = general_purpose::STANDARD.encode(&result_bytes);

    return Ok(result);
}