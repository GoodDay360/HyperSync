use axum::{
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::{Next},
    response::Response,
};

use crate::models::user::AUTH_USER;

pub async fn new(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = headers.get("authorization")
        .ok_or("Missing user token.")
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;


    match AUTH_USER::verify(&token) {
        Ok(user_state) => {
            if user_state.is_none() {
                return Err(StatusCode::UNAUTHORIZED);
            }
            let response = next.run(request).await;
            return Ok(response);
        }
        Err(_) => {
            AUTH_USER::add(&token).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
            let response = next.run(request).await;
            return Ok(response);
        }
    };  
}