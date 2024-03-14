
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::Deserialize;
use actix_web::{web, HttpResponse, Responder, error};
use std::error::Error;
use sqlx::{PgPool, Row};
use crate::AppState;
use crate::db::fetch_id::fetch_id;

#[derive(Clone, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String
}
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String
}

impl NewUser {
    pub fn new (username: String, password: String, email: String) -> NewUser {
        let hashed_password: String = hash (password.as_str(), DEFAULT_COST).unwrap();
        return NewUser {
            username,
            password: hashed_password,
            email
        }
    }
}
impl User {
    pub fn verify(&self, password: String) -> bool {
        verify(password.as_str(), &self.password).unwrap()
    }
}
pub async fn insert( st: NewUser, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let id = fetch_id("users".to_string(), pool).await;
    let todo = sqlx::query("INSERT INTO users (id, username, password, email, admin) VALUES ($1, $2, $3, $4, 0)")
        .bind(id)
        .bind(&st.username)
        .bind(&st.password)
        .bind(&st.email)
        .execute(pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()));
    match todo {
        Ok(_) => {
            println!("OK");
            Ok(())
        }
        Err(e) => {
            println!("Err");
            println!("{e}");
            Err(Box::try_from(e).unwrap())
        }
    }
}
pub async fn register (new_user: web::Json<NewUser>, state: web::Data<AppState>) -> impl Responder {
    let new_user = NewUser::new(
        new_user.username.clone(),
        new_user.password.clone(),
        new_user.email.clone()
    );
    println!("{} {}", new_user.password, new_user.username);
    match insert(new_user, &state.pool).await {
        Ok(_) => {
            println!("Created");
            HttpResponse::Created()
        },
        Err(_) => {
            println!("Conflict");
            HttpResponse::Conflict()
        }
    }
}