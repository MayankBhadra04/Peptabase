use bcrypt::{ verify};
use serde_json;
#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String
}

use actix_web::{web, HttpResponse, error};
use actix_web::cookie::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::auth::jwt::{JwToken};
use crate::db::structs::Users;

pub async fn login (credentials: web::Json<Login>, state: web::Data<AppState>) -> HttpResponse {
    let password = &credentials.password;
    let mut todo: Vec<Users> = sqlx::query_as("SELECT password FROM users WHERE email = $1")
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
            let token = JwToken::new(credentials.email.clone(), &state.pool);
            let raw_token = token.await.encode();
            println!("{}", raw_token);
            let response = LoginResponse{token: raw_token.clone()};
            let body = serde_json::to_string(&response).unwrap();
            println!("{body}");
            let mut cookie = Cookie::new("jwt", raw_token);
            cookie.set_http_only(true); // Set HttpOnly attribute
            cookie.set_secure(false); // Set Secure attribute
            cookie.set_same_site(SameSite::Strict);
            cookie.set_path("/");
            HttpResponse::Ok()
                .cookie(cookie)
                .append_header(("Access-Control-Allow-Origin", "http://localhost:3000"))
                .append_header(("Access-Control-Allow-Credentials", "true"))
                .finish()
        },
        false => {
            HttpResponse::Unauthorized().into()
        }
    }
}