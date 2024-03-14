use bcrypt::{ verify};
use serde_json;
#[derive(Deserialize, FromRow)]
pub struct Login {
    pub email: String,
    pub password: String
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String
}

use actix_web::{web, HttpResponse, error};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::AppState;
use crate::auth::jwt::{JwToken};

pub async fn login (credentials: web::Json<Login>, state: web::Data<AppState>) -> HttpResponse {
    let password = &credentials.password;
    let mut todo: Vec<Login> = sqlx::query_as("SELECT email, password FROM users WHERE email = $1")
        .bind(&credentials.email)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string())).unwrap();
    if todo.len() == 0 {
        return HttpResponse::NotFound().await.unwrap();
    } else if todo.len()>1 {
        return HttpResponse::Conflict().await.unwrap();
    }
    let rows = todo.pop().unwrap();
    match verify(password, &rows.password).unwrap() {
        true =>{
            let token = JwToken::new(rows.email, &state.pool);
            let raw_token = token.await.encode();
            let response = LoginResponse{token: raw_token.clone()};
            let body = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().append_header(("token", raw_token)).json(&body)
        },
        false => {
            HttpResponse::Unauthorized().into()
        }
    }
}