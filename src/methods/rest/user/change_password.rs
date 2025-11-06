
use axum::{
    response::{Json as JsonResponse},
    extract::Json,
    http::{HeaderMap},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};
use sea_orm::{
    EntityTrait, QueryFilter, ColumnTrait, QuerySelect,
    sea_query::Expr
};


use crate::entities::user;
use crate::utils::database;
use crate::models::error::ErrorResponse;
use crate::utils::{encrypt, decrypt};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    current_password: String,
    new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: bool,
    token: String
}


pub async fn new(headers: HeaderMap, Json(payload): Json<Payload>) -> Result<JsonResponse<Response>, ErrorResponse>{
    let token = headers.get("authorization")
        .ok_or("Missing user token.")
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?
        .to_str()
        .map_err(|e| ErrorResponse{status:500, message: e.to_string()})?;
    
    /* Verify Current and New Password */
    if payload.current_password == payload.new_password {
        return Err(ErrorResponse{status: 500, message: "Current and new password cannot be the same.".to_string()});
    }

    if payload.new_password.len() < 8 {
        return Err(ErrorResponse{status: 500, message: "Password too short.".to_string()});
    }
    if payload.current_password.len() > 32 || payload.new_password.len() > 32 {
        return Err(ErrorResponse{status: 500, message: "Password too long.".to_string()});
    }
    
    /* --- */

    let conn = database::get_connection().await
        .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    let result = user::Entity::find()
        .select_only()
        .column(user::Column::Password)
        .column(user::Column::Id)
        .column(user::Column::Email)
        .filter(user::Column::Token.eq(token))
        .into_tuple::<(String, String, String)>()
        .one(&conn)
        .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

    let is_auth: bool;
    let user_id: String;
    let user_email: String;
    if let Some((password, id, email)) = result {
        let decrypt_password = String::from_utf8(
            decrypt::new(&password)
                .map_err(|e| ErrorResponse{status: 500, message: e})?
        )
            .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

        if decrypt_password == payload.current_password {
            is_auth = true;
            user_id = id;
            user_email = email;
        }else{
            return Err(ErrorResponse{status: 500, message: "Incorrect current password.".to_string()})?;
        }
    }else{
        return Err(ErrorResponse{status: 500, message: "User not found.".to_string()})?;
    }

    if is_auth && !user_id.is_empty() && !user_email.is_empty() {
        /* Create New Token */
        let creds = json!({
            "id": &user_id,
            "email": &user_email,
            "password": &payload.new_password
        });

        let creds_to_string = to_string(&creds)
            .map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

        let new_token = encrypt::new(&creds_to_string.as_bytes())
            .map_err(|e| ErrorResponse{status: 500, message: e})?;
        /* --- */

        let new_encrypted_password = encrypt::new(&payload.new_password.as_bytes())
            .map_err(|e| ErrorResponse{status: 500, message: e})?;

        user::Entity::update_many()
            .col_expr(user::Column::Token, Expr::value(&new_token))
            .col_expr(user::Column::Password, Expr::value(&new_encrypted_password))
            .filter(user::Column::Id.eq(user_id))
            .exec(&conn)
            .await.map_err(|e| ErrorResponse{status: 500, message: e.to_string()})?;

        return Ok(JsonResponse(Response{status: true, token: new_token.to_string()}));
        
    }else{
        return Err(ErrorResponse{status: 500, message: "Invalid user token.".to_string()})?;
    }
}